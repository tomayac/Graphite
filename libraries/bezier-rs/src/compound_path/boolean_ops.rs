/*
 * Paper.js - The Swiss Army Knife of Vector Graphics Scripting.
 * http://paperjs.org/
 *
 * Copyright (c) 2011 - 2020, Jürg Lehni & Jonathan Puckey
 * http://juerglehni.com/ & https://puckey.studio/
 *
 * Distributed under the MIT license. See LICENSE file for details.
 *
 * All rights reserved.
 */

/*
 * Boolean Geometric Path Operations
 *
 * Supported
 *  - Path and CompoundPath items
 *  - Boolean Union
 *  - Boolean Intersection
 *  - Boolean Subtraction
 *  - Boolean Exclusion
 *  - Resolving a self-intersecting Path items
 *  - Boolean operations on self-intersecting Paths items
 *
 * @author Harikrishnan Gopalakrishnan <hari.exeption@gmail.com>
 * @author Jan Boesenberg <jan.boesenberg@gmail.com>
 * @author Jürg Lehni <juerg@scratchdisk.com>
 */

// Set up lookup tables for each operator, to decide if a given segment
// is to be considered a part of the solution, or to be discarded, based
// on its winding contribution, as calculated by propagateWinding().
// Boolean operators return true if a segment with the given winding
// contribution contributes to the final result or not. They are applied
// to for each segment after the paths are split at crossings.
// operators = {
// 	unite:     { "1": true, "2": true },
// 	intersect: { "2": true },
// 	subtract:  { "1": true },
// 	// exclude only needs -1 to support reorientPaths() when there are
// 	// no crossings. The actual boolean code uses unsigned winding.
// 	exclude:   { "1": true, "-1": true }
// };

use crate::compound_path::collision_detection::*;
use crate::compound_path::numerical::*;

enum Operator {
	Unite,
	Intersect,
	Subtract,
	Exclude,
	Divide,
}

struct BooleanOperationOptions {
	trace: bool,
	stroke: bool,
}

struct Point {
	x: f64,
	y: f64,
	length: f64,
	angle: f64,
	angleInRadians: f64,
	// quadrant, selected
}

struct Curve<'a> {
	point1: Point,
	point2: Point,
	handle1: Point,
	handle2: Point,
	segment1: Segment,
	segment2: Segment,
	path: Path<'a>,
	index: usize,
	next: &'a Curve<'a>,
	prev: &'a Curve<'a>,
	// values, points, length, area, bounds, strokeBounds, handleBounds
}

struct Path<'a> {
	segments: Vec<Segment>,
	firstSegment: Segment,
	lastSegment: Segment,
	curves: Vec<&'a Curve<'a>>,
	firstCurve: &'a Curve<'a>,
	lastCurve: &'a Curve<'a>,
	closed: bool,
	// length, area, fullySelected
}

struct Segment {
	point: Point,
	handleIn: Point,
	handleOut: Point,
	// selected, index, path, curve, location, next, previous
}

struct WindingInfo {
	winding: f64,
	windingL: f64,
	windingR: f64,
	quality: f64,
	onPath: bool,
}

fn operators_lookup_table(operator: Operator, key: i32) -> bool {
	match operator {
		Operator::Unite => match key {
			1 | 2 => true,
			_ => false,
		},
		Operator::Intersect => match key {
			2 => true,
			_ => false,
		},
		Operator::Subtract => match key {
			1 => true,
			_ => false,
		},
		// exclude only needs -1 to support reorientPaths() when there are no crossings. The actual boolean code uses unsigned winding.
		Operator::Exclude => match key {
			1 => true,
			-1 => true,
			_ => false,
		},
		Operator::Divide => false,
	}
}

impl PathItem {
	pub fn getBounds(matrix, options) {
		let hasMatrix = options || matrix instanceof Matrix;
		let opts = Base.set({}, if hasMatrix { options } else { matrix }, this._boundsOptions);
		// We can only cache the bounds if the path uses stroke-scaling, or if
		// no stroke is involved in the calculation of the bounds.
		// When strokeScaling is false, the bounds are affected by the zoom
		// level of the view, hence we can't cache.
		// TODO: Look more into handling of stroke-scaling, e.g. on groups with
		// some children that have strokeScaling, as well as SymbolItem with
		// SymbolDefinition that have strokeScaling!
		// TODO: Once that is resolved, we should be able to turn off
		// opts.stroke if a resolved item definition does not have a stroke,
		// allowing the code to share caches between #strokeBounds and #bounds.
		if !opts.stroke || this.getStrokeScaling() {
			opts.cacheItem = this;
		}
		// If we're caching bounds, pass on this item as cacheItem, so
		// the children can setup _boundsCache structures for it.
		let rect = this._getCachedBounds(hasMatrix && matrix, opts).rect;
		// If we're returning '#bounds', create a LinkedRectangle that uses the
		// setBounds() setter to update the Item whenever the bounds are changed:
		if arguments.is_empty() {
			new LinkedRectangle(rect.x, rect.y, rect.width, rect.height, this, "setBounds")
		} else {
			rect
		}
	}
}

fn getPaths(path) {
	return path._children || [path];
}

#[derive(Clone, Eq, PartialEq)]
struct PathItem {
	id: i32,
	className: String,
	name: String,
	// style, locked, visible, blendMode, opacity, selected, clipMask, data, position, pivot, bounds, strokeBounds, handleBounds, internalBounds, rotation, scaling, matrix, globalMatrix, viewMatrix, applyMatrix, project, view, layer, parent, children, firstChild, lastChild, nextSibling, previousSibling, index, strokeColor, strokeWidth, strokeCap, strokeJoin, dashOffset, strokeScaling, dashArray, miterLimit, fillColor, fillRule, shadowColor, shadowBlur, shadowOffset, selectedColor, onFrame, onMouseDown, onMouseDrag, onMouseUp, onClick, onDoubleClick, onMouseMove, onMouseEnter, onMouseLeave
}

impl PathItem {
	/// Creates a clone of the path that we can modify freely, with its matrix
	/// applied to its geometry. Calls #reduce() to simplify compound paths and
	/// remove empty curves, #resolveCrossings() to resolve self-intersection
	/// make sure all paths have correct winding direction.
	fn preparePath(path, resolve) {
		let res = path
			.clone(false)
			.reduce({ simplify: true })
			.transform(null, true, true);
		if resolve {
			// For correct results, close open paths with straight lines:
			let paths = getPaths(res);
			for _ in 0..paths.len() {
				let path = paths[i];
				if (!path._closed && !path.isEmpty()) {
					// Close with epsilon tolerance, to avoid tiny straight
					// that would cause issues with intersection detection.
					path.closePath(Numerical::EPSILON);
					path.getFirstSegment().setHandleIn(0, 0);
					path.getLastSegment().setHandleOut(0, 0);
				}
			}
			res = res
				.resolveCrossings()
				.reorient(res.getFillRule() == "nonzero", true);
		}
		return res;
	}

	fn createResult(paths, simplify, path1, path2, options) {
		let result = new CompoundPath(PathItem.NO_INSERT);
		result.addChildren(paths, true);
		// See if the item can be reduced to just a simple Path.
		result = result.reduce({ simplify: simplify });
		if (!(options && options.insert == false)) {
			// Insert the resulting path above whichever of the two paths appear
			// further up in the stack.
			result.insertAbove(path2 && path1.isSibling(path2) && if path1.getIndex() < path2.getIndex() { path2 } else { path1 });
		}
		// Copy over the input path attributes, excluding matrix and we're done.
		result.copyAttributes(path1, true);
		return result;
	}

	fn filterIntersection(inter) {
		// TODO: Change isCrossing() to also handle overlaps (hasOverlap())
		// that are actually involved in a crossing! For this we need proper
		// overlap range detection / merging first... But as we call
		// #resolveCrossings() first in boolean operations, removing all
		// self-touching areas in paths, this works for the known use cases.
		// The ideal implementation would deal with it in a way outlined in:
		// https://github.com/paperjs/paper.js/issues/874#issuecomment-168332391
		return inter.hasOverlap() || inter.isCrossing();
	}

	fn traceBoolean(path1: PathItem, path2: PathItem, operation: Operator, options: BooleanOperationOptions) {
		// Only support subtract and intersect operations when computing stroke
		// based boolean operations (options.split = true).
		if (options.trace == false || options.stroke) && matches!(operation, Operator::Subtract | Operator::Intersect) {
			return splitBoolean(path1, path2, operation);
		}
		// We do not modify the operands themselves, but create copies instead,
		// fas produced by the calls to preparePath().
		// NOTE: The result paths might not belong to the same type i.e.
		// subtract(A:Path, B:Path):CompoundPath etc.
		let _path1 = preparePath(path1, true);
		let _path2 = path2 && path1 != path2 && preparePath(path2, true);
		// Retrieve the operator lookup table for winding numbers.
		// let operator = operators_lookup_table(operation);
		// Add a simple boolean property to check for a given operation, e.g. `if (operator.unite)`
		// operator[operation] = true;

		// Give both paths the same orientation except for subtraction
		// and exclusion, where we need them at opposite orientation.
		if _path2 && matches!(operation, Operator::Subtract | Operator::Exclude) ^ (_path2.isClockwise() ^ _path1.isClockwise()) {
			_path2.reverse();
		}
		// Split curves at crossings on both paths. Note that for self-
		// intersection, path2 is null and getIntersections() handles it.
		let crossings = divideLocations(CurveLocation.expand(_path1.getIntersections(_path2, filterIntersection)));
		let paths1 = getPaths(_path1);
		let paths2 = _path2 && getPaths(_path2);
		let segments = [];
		let curves: Vec<Curve> = vec![];
		let mut paths;

		fn collectPaths(paths) {
			for path in paths {
				Base.push(segments, path._segments);
				Base.push(curves, path.getCurves());
				// See if all encountered segments in a path are overlaps, to
				// be able to separately handle fully overlapping paths.
				path._overlapsOnly = true;
			}
		}

		let getCurves = |indices| {
			// let list = [];
			// for index in indices {
			// 	list.push(curves[index]);
			// }
			// return list;
			indices.iter().map(|index| curves[*index]).collect::<Vec<_>>()
		};

		if crossings.len() {
			// Collect all segments and curves of both involved operands.
			collectPaths(paths1);
			if paths2 {
				collectPaths(paths2);
			}

			let curvesValues = curves.iter().map(|curve| curve.getValues()).collect::<Vec<_>>();
			let curveCollisions = CollisionDetection.findCurveBoundsCollisions(curvesValues, curvesValues, 0, true);
			let curveCollisionsMap = {};
			// for (let i = 0; i < curves.length; i++) {
			for (i, curve) in curves.iter().enumerate() {
				let id = curve._path._id;
				curveCollisionsMap[id] = curveCollisionsMap[id] || {};
				let map = curveCollisionsMap[id];
				map[curve.getIndex()] = {
					hor: getCurves(curveCollisions[i].hor),
					ver: getCurves(curveCollisions[i].ver)
				};
			}

			// Propagate the winding contribution. Winding contribution of
			// curves does not change between two crossings.
			// First, propagate winding contributions for curve chains starting
			// in all crossings:
			for crossing in crossings {
				Self::propagateWinding(crossing._segment, _path1, _path2, curveCollisionsMap, operation);
			}
			for segment in segments {
				let inter = segment._intersection;
				if !segment._winding {
					Self::propagateWinding(segment, _path1, _path2, curveCollisionsMap, operation);
				}
				// See if all encountered segments in a path are overlaps.
				if !(inter && inter._overlap) {
					segment._path._overlapsOnly = false;
				}
			}
			paths = Self::tracePaths(segments, operation);
		} else {
			// When there are no crossings, the result can be determined through
			// a much faster call to reorientPaths():
			paths = Self::reorientPaths(
					// Make sure reorientPaths() never works on original _children arrays by calling paths1.clone()
					if paths2 { paths1.concat(paths2) } else { paths1.clone() },
					|w: i32| operators_lookup_table(operation, w),
					None
				);
		}
		return Self::createResult(paths, true, path1, path2, options);
	}

	fn splitBoolean(path1, path2, operation) {
		let _path1 = preparePath(path1);
		let _path2 = preparePath(path2);
		let crossings = _path1.getIntersections(_path2, filterIntersection);
		let subtract = operation == Operator::Subtract;
		let divide = operation == Operator::Divide;
		let added = {};
		let paths = [];

		let addPath = |path| {
			// Simple see if the point halfway across the open path is inside
			// path2, and include / exclude the path based on the operator.
			if !added[path._id] && (divide || _path2.contains(path.getPointAt(path.getLength() / 2)) ^ subtract) {
				paths.unshift(path);
				added[path._id] = true;
				true
			} else {
				false
			}
		};

		// Now loop backwards through all crossings, split the path and check
		// the new path that was split off for inclusion.
		// for (let i = crossings.length - 1; i >= 0; i--) {
		for path in crossings.iter().rev() {
			let path = path.split();
			if path {
				// See if we can add the path, and if so, clear the first handle
				// at the split, because it might have been a curve.
				if addPath(path) {
					path.getFirstSegment().setHandleIn(0, 0);
				}
				// Clear the other side of the split too, which is always the
				// end of the remaining _path1.
				_path1.getLastSegment().setHandleOut(0, 0);
			}
		}
		// At the end, add what's left from our path after all the splitting.
		addPath(_path1);
		return createResult(paths, false, path1, path2);
	}

	/// Creates linked lists between intersections through their _next and _prev properties.
	///
	/// @private
	fn linkIntersections(from, to) {
		// Only create the link if it's not already in the existing chain, to
		// avoid endless recursions. First walk to the beginning of the chain,
		// and abort if we find `to`.
		let prev = from;
		while prev {
			if prev == to {
				return;
			}
			prev = prev._previous;
		}
		// Now walk to the end of the existing chain to find an empty spot, but
		// stop if we find `to`, to avoid adding it again.
		while from._next && from._next != to {
			from = from._next;
		}
		// If we're reached the end of the list, we can add it.
		if !from._next {
			// Go back to beginning of the other chain, and link the two up.
			while to._previous {
				to = to._previous;
			}
			from._next = to;
			to._previous = from;
		}
	}

	fn clearCurveHandles(curves) {
		// Clear segment handles if they were part of a curve with no handles.
		// for (let i = curves.length - 1; i >= 0; i--)
		for curve in curves.iter().rev() {
			curve.clearHandles();
		}
	}

	/// Reorients the specified paths.
	///
	/// @param {Item[]} paths the paths of which the orientation needs to be reoriented
	/// @param {Function} isInside determines if the inside of a path is filled.
	///     For non-zero fill rule this function would be implemented as follows:
	///
	///     fn isInside(w) { w != 0 }
	/// @param {Boolean} [clockwise] if provided, the orientation of the root
	///     paths will be set to the orientation specified by `clockwise`,
	///     otherwise the orientation of the largest root child is used.
	/// @return {Item[]} the reoriented paths
	fn reorientPaths(paths: &[PathItem], isInside: bool, clockwise: Option<bool>) -> &[PathItem] {
		let length = paths.len();
		if length > 0 {
			let lookup = Base.each(
				// obj:
				paths,
				// iter:
				|path, i| {
					// Build a lookup table with information for each path's original index and winding contribution.
					this[path._id] = {
						container: null,
						winding: path.isClockwise() ? 1 : -1,
						index: i,
					};
				},
				// bind:
				{},
			);
			// Now sort the paths by their areas, from large to small.
			let mut sorted = paths.to_vec();
			sorted.sort_by(|a, b| b.getArea().abs() - a.getArea().abs());
			// Get reference to the first, largest path and insert it already.
			let first = sorted[0];
			// create lookup containing potentially overlapping path bounds
			let collisions = CollisionDetection.findItemBoundsCollisions(sorted.as_slice(), &[], Some(Numerical::GEOMETRIC_EPSILON));
			let clockwise = if clockwise == None {
				first.isClockwise()
			};
			// Now determine the winding for each path, from large to small.
			for i in 0..length {
				let path1 = sorted[i];
				let entry1 = lookup[path1._id];
				let containerWinding = 0;
				let indices = collisions.get(i);
				if let Some(indices) = indices {
					let mut point = None; // interior point, only get it if required.
					// for (let j = indices.length - 1; j >= 0; j--) {
					for &index in indices.iter().rev() {
						if index < i {
							point = if point.is_none() { Some(path1.getInteriorPoint()) } else { point };
							let path2 = sorted[index];
							// As we run through the paths from largest to
							// smallest, for any current path, all potentially
							// containing paths have already been processed and
							// their orientation fixed. To achieve correct
							// orientation of contained paths based on winding,
							// find one containing path with different
							// "insideness" and set opposite orientation.
							if path2.contains(point) {
								let entry2 = lookup[path2._id];
								containerWinding = entry2.winding;
								entry1.winding += containerWinding;
								entry1.container = if entry2.exclude { entry2.container } else { path2 };
								break;
							}
						}
					}
				}
				// Only keep paths if the "insideness" changes when crossing the
				// path, e.g. the inside of the path is filled and the outside
				// is not, or vice versa.
				if isInside(entry1.winding) == isInside(containerWinding) {
					entry1.exclude = true;
					// No need to delete excluded entries. Setting to null is
					// enough, as #setChildren() can handle arrays with gaps.
					paths[entry1.index] = null;
				} else {
					// If the containing path is not excluded, we're done
					// searching for the orientation defining path.
					let container = entry1.container;
					path1.setClockwise(if container { !container.isClockwise() } else { clockwise });
				}
			}
		}
		paths
	}

	/// Divides the path-items at the given locations.
	///
	/// @param {CurveLocation[]} locations an array of the locations to split the
	///     path-item at.
	/// @param {Function} [include] a function that determines if dividing should
	///     happen at a given location.
	/// @return {CurveLocation[]} the locations at which the involved path-items
	///     were divided
	/// @private
	fn divideLocations(locations, include, clearLater) {
		let results = include && [];
		let tMin = Numerical::CURVETIME_EPSILON;
		let tMax = 1 - tMin;
		let clearHandles = false;
		let clearCurves = clearLater || [];
		let mut clearLookup = clearLater && {};
		let renormalizeLocs;
		let prevCurve;
		let prevTime;

		// When dealing with overlaps and crossings, divideLocations() is called
		// twice. If curve handles of curves that originally didn't have handles
		// are cleared after the first call , we loose  curve-time consistency
		// and CurveLocation#_time values become invalid.
		// In those situations, clearLater is passed as a container for all
		// curves of which the handles need to be cleared in the end.
		// Create a lookup table that allows us to quickly determine if a given
		// curve was resulting from an original curve without handles.
		fn getId(curve) {
			return curve._path._id + "." + curve._segment1._index;
		}

		// for (let i = (clearLater && clearLater.length) - 1; i >= 0; i--) {
		for curve in clearLater.iter().rev() {
			if curve._path {
				clearLookup[getId(curve)] = true;
			}
		}

		// Loop backwards through all sorted locations, from right to left, so
		// we can assume a predefined sequence for curve-time renormalization.
		// for (let i = locations.length - 1; i >= 0; i--) {
		for loc in locations.iter().rev() {
			// Retrieve curve-time before calling include(), because it may
			// be changed to the scaled value after splitting previously.
			// See CurveLocation#getCurve(), #resolveCrossings()
			let time = loc._time;
			let origTime = time;
			let exclude = include && !include(loc);
			// Retrieve curve after calling include(), because it may cause
			// a change in the cached location values, see above.
			let curve = loc._curve;
			let segment;
			if (curve) {
				if (curve != prevCurve) {
					// This is a new curve, update clearHandles setting.
					clearHandles = !curve.hasHandles() || clearLookup && clearLookup[getId(curve)];
					// Keep track of locations for later curve-time
					// renormalization within the curve.
					renormalizeLocs = [];
					prevTime = null;
					prevCurve = curve;
				} else if (prevTime >= tMin) {
					// Rescale curve-time when we are splitting the same curve
					// multiple times, if splitting was done previously.
					time /= prevTime;
				}
			}
			if (exclude) {
				// Store excluded locations for later renormalization, in case the same curve is divided to their left.
				if renormalizeLocs {
					renormalizeLocs.push(loc);
				}
				continue;
			} else if (include) {
				results.unshift(loc);
			}
			prevTime = origTime;
			if (time < tMin) {
				segment = curve._segment1;
			} else if (time > tMax) {
				segment = curve._segment2;
			} else {
				// Split the curve at time, passing true for _setHandles to
				// always set the handles on the sub-curves even if the original
				// curve had no handles.
				let newCurve = curve.divideAtTime(time, true);
				// Keep track of curves without handles, so they can be cleared
				// again at the end.
				if clearHandles {
					clearCurves.push(curve, newCurve);
				}
				segment = newCurve._segment1;
				// Handle locations that need their curve-time renormalized
				// within the same curve after dividing at this location.
				// for (let j = renormalizeLocs.length - 1; j >= 0; j--) {
				for _ in renormalizeLocs.iter().rev() {
					let l = renormalizeLocs[j];
					l._time = (l._time - time) / (1 - time);
				}
			}
			loc._setSegment(segment);
			// Create links from the new segment to the intersection on the
			// other curve, as well as from there back. If there are multiple
			// intersections on the same segment, we create linked lists between
			// the intersections through linkIntersections(), linking both ways.
			let inter = segment._intersection,
				dest = loc._intersection;
			if inter {
				linkIntersections(inter, dest);
				// Each time we add a new link to the linked list, we need to
				// add links from all the other entries to the new entry.
				let other = inter;
				while (other) {
					linkIntersections(other._intersection, inter);
					other = other._next;
				}
			} else {
				segment._intersection = dest;
			}
		}
		// Clear curve handles right away if we're not storing them for later.
		if !clearLater {
			clearCurveHandles(clearCurves);
		}
		return results || locations;
	}

	/// Returns the winding contribution number of the given point in respect
	/// to the shapes described by the passed curves.
	///
	/// See #1073#issuecomment-226942348 and #1073#issuecomment-226946965 for a
	/// detailed description of the approach developed by @iconexperience to
	/// precisely determine the winding contribution in all known edge cases.
	///
	/// @param {Point} point the location for which to determine the winding contribution
	/// @param {Curve[]} curves The curves that describe the shape against which to check, as returned by {@link Path#curves} or {@link CompoundPath#curves}.
	/// @param {Boolean} [dir=false] the direction in which to determine the winding contribution, `false`: in x-direction, `true`: in y-direction
	/// @param {Boolean} [closed=false] determines how areas should be closed when a curve is part of an open path, `false`:
	///     area is closed with a straight line, `true`: area is closed taking the handles of the first and last segment into account
	/// @param {Boolean} [dontFlip=false] controls whether the algorithm is allowed to flip direction if it is deemed to produce better results
	/// @return {Object} an object containing the calculated winding number, as well as an indication whether the point was situated on the contour
	/// @private
	fn getWinding(point: Point, curves: &[Curve], dir: bool, closed: bool, dontFlip: bool) -> Option<WindingInfo> {
		let curvesList = curves;
		// Determine the index of the abscissa and ordinate values in the curve values arrays, based on the direction:
		let ia = if dir { 1 } else { 0 }; // the abscissa index
		let io = ia ^ 1; // the ordinate index
		let pv = [point.x, point.y];
		let pa = pv[ia]; // the point's abscissa
		let po = pv[io]; // the point's ordinate
		// Use separate epsilons for winding contribution code.
		let windingEpsilon = 1e-9;
		let qualityEpsilon = 1e-6;
		let paL = pa - windingEpsilon;
		let paR = pa + windingEpsilon;
		let windingL = 0;
		let windingR = 0;
		let mut pathWindingL = 0;
		let mut pathWindingR = 0;
		let mut onPath = false;
		let onAnyPath = false;
		let mut quality = 1.;
		let roots = [];
		let vPrev;
		let vClose;

		let addWinding = |v| {
			let o0 = v[io + 0];
			let o3 = v[io + 6];
			if (po < min(o0, o3) || po > max(o0, o3)) {
				// If the curve is outside the ordinates' range, no intersection with the ray is possible.
				return None;
			}
			let a0 = v[ia + 0];
			let a1 = v[ia + 2];
			let a2 = v[ia + 4];
			let a3 = v[ia + 6];
			if (o0 == o3) {
				// A horizontal curve is not necessarily between two non-horizontal curves. We have to take cases like these into account:
				//          +-----+
				//     +----+     |
				//          +-----+
				if (a0 < paR && a3 > paL || a3 < paR && a0 > paL) {
					onPath = true;
				}
				// If curve does not change in ordinate direction, windings will be added by adjacent curves.
				// Bail out without updating vPrev at the end of the call.
				return None;
			}
			// Determine the curve-time value corresponding to the point.
			let t = if po == o0 {
					0
				} else {
					if po == o3 {
						1
					}
					// If the abscissa is outside the curve, we can use any
					// value except 0 (requires special handling). Use 1, as it
					// does not require additional calculations for the point.
					else {
						if paL > max(a0, a1, a2, a3) || paR < min(a0, a1, a2, a3) {
							1
						} else {
							if Curve.solveCubic(v, io, po, roots, 0, 1) > 0 { roots[0] } else { 1 }
						}
					}
				};
			let a = if t == 0 { a0 } else {
				if t == 1 { a3 } else { Curve.getPoint(v, t)[if dir { "y" } else { "x" }]}
			};
			let winding = if o0 > o3 { 1 } else { -1 };
			let windingPrev = if vPrev[io] > vPrev[io + 6] { 1 } else { -1 };
			let a3Prev = vPrev[ia + 6];
			if po != o0 {
				// Standard case, curve is not crossed at its starting point.
				if a < paL {
					pathWindingL += winding;
				} else if a > paR {
					pathWindingR += winding;
				} else {
					onPath = true;
				}
				// Determine the quality of the winding calculation. Reduce the
				// quality with every crossing of the ray very close to the
				// path. This means that if the point is on or near multiple
				// curves, the quality becomes less than 0.5.
				if a > pa - qualityEpsilon && a < pa + qualityEpsilon {
					quality /= 2.;
				}
			} else {
				// Curve is crossed at starting point.
				if (winding != windingPrev) {
					// Winding changes from previous curve, cancel its winding.
					if (a0 < paL) {
						pathWindingL += winding;
					} else if (a0 > paR) {
						pathWindingR += winding;
					}
				} else if (a0 != a3Prev) {
					// Handle a horizontal curve between the current and
					// previous non-horizontal curve. See
					// #1261#issuecomment-282726147 for a detailed explanation:
					if a3Prev < paR && a > paR {
						// Right winding was not added before, so add it now.
						pathWindingR += winding;
						onPath = true;
					} else if a3Prev > paL && a < paL {
						// Left winding was not added before, so add it now.
						pathWindingL += winding;
						onPath = true;
					}
				}
				quality /= 4.;
			}
			vPrev = v;

			// If we're on the curve, look at the tangent to decide whether to flip direction to better determine a reliable winding number:
			// If the tangent is parallel to the direction, call getWinding() again with flipped direction and return that result instead.
			if !dontFlip && a > paL && a < paR && Curve.getTangent(v, t)[if dir { "x" } else { "y" }] == 0 {
				getWinding(point, curves, !dir, closed, true)
			} else {
				None
			}
		};

		let handleCurve = |v| {
			// Get the ordinates:
			let o0 = v[io + 0];
			let o1 = v[io + 2];
			let o2 = v[io + 4];
			let o3 = v[io + 6];
			// Only handle curves that can cross the point's ordinate.
			if po <= max(o0, o1, o2, o3) && po >= min(o0, o1, o2, o3) {
				// Get the abscissas:
				let a0 = v[ia + 0];
				let a1 = v[ia + 2];
				let a2 = v[ia + 4];
				let a3 = v[ia + 6];
				// Get monotone curves. If the curve is outside the point's abscissa, it can be treated as a monotone curve:
				let monoCurves = if paL > max(a0, a1, a2, a3) || paR < min(a0, a1, a2, a3) { [v] } else { Curve.getMonoCurves(v, dir) };
				// for (let i = 0; i < monoCurves.length; i++) {
				for monoCurve in monoCurves {
					// Calling addWinding() my lead to direction flipping, in which case we already have the result and can return it.
					let res = addWinding(monoCurve);
					if res.is_some() {
						return res;
					}
				}
			}
			None
		};

		// for (let i = 0; i < curvesList.length; i++) {
		for (i, curve) in curvesList.iter().enumerate() {
			let l = curvesList.len();
			let path = curve._path;
			let v = curve.getValues();
			if !(i as bool) || curvesList[i - 1]._path != path {
				// We're on a new (sub-)path, so we need to determine values of
				// the last non-horizontal curve on this path.
				vPrev = null;
				// If the path is not closed, connect the first and last segment
				// based on the value of `closed`:
				// - `false`: Connect with a straight curve, just like how
				//   filling open paths works.
				// - `true`: Connect with a curve that takes the segment handles
				//   into account, just like how closed paths behave.
				if !path._closed {
					vClose = Curve.getValues(path.getLastCurve().getSegment2(), curve.getSegment1(), null, !closed);
					// This closing curve is a potential candidate for the last
					// non-horizontal curve.
					if vClose[io] != vClose[io + 6] {
						vPrev = vClose;
					}
				}

				if !vPrev {
					// Walk backwards through list of the path's curves until we
					// find one that is not horizontal.
					// Fall-back to the first curve's values if none is found:
					vPrev = v;
					let prev = path.getLastCurve();
					while prev && prev != curve {
						let v2 = prev.getValues();
						if v2[io] != v2[io + 6] {
							vPrev = v2;
							break;
						}
						prev = prev.getPrevious();
					}
				}
			}

			// Calling handleCurve() my lead to direction flipping, in which
			// case we already have the result and can return it.
			let mut res = handleCurve(v);
			if res.is_some() {
				return res;
			}

			if (i + 1 == l || curvesList[i + 1]._path != path) {
				// We're at the last curve of the current (sub-)path. If a
				// closing curve was calculated at the beginning of it, handle
				// it now to treat the path as closed:
				if vClose {
					res = handleCurve(vClose);
					if res.is_some() {
						return res;
					}
				}
				if onPath && !pathWindingL && !pathWindingR {
					// If the point is on the path and the windings canceled
					// each other, we treat the point as if it was inside the
					// path. A point inside a path has a winding of [+1,-1]
					// for clockwise and [-1,+1] for counter-clockwise paths.
					// If the ray is cast in y direction (dir == true), the
					// windings always have opposite sign.
					pathWindingR = if path.isClockwise(closed) ^ dir { 1 } else { -1 };
					pathWindingL = pathWindingR;
				}
				windingL += pathWindingL;
				windingR += pathWindingR;
				pathWindingL = pathWindingR = 0;
				if onPath {
					onAnyPath = true;
					onPath = false;
				}
				vClose = null;
			}
		}
		// Use the unsigned winding contributions when determining which areas
		// are part of the boolean result.
		windingL = abs(windingL);
		windingR = abs(windingR);
		// Return the calculated winding contributions along with a quality
		// value indicating how reliable the value really is.
		Some(WindingInfo {
			winding: max(windingL, windingR),
			windingL: windingL,
			windingR: windingR,
			quality: quality,
			onPath: onAnyPath,
		})
	}

	fn propagateWinding(segment, path1, path2, curveCollisionsMap, operation: Operator) {
		// Here we try to determine the most likely winding number contribution
		// for the curve-chain starting with this segment. Once we have enough
		// confidence in the winding contribution, we can propagate it until the
		// next intersection or end of a curve chain.
		let chain = [];
		let start = segment;
		let totalLength = 0;
		let winding;
		loop {
			let curve = segment.getCurve();
			// We can encounter paths with only one segment, which would not have a curve.
			if (curve) {
				let length = curve.getLength();
				chain.push({ segment: segment, curve: curve, length: length });
				totalLength += length;
			}
			segment = segment.getNext();

			if !(segment && !segment._intersection && segment != start) {
				break;
			}
		}
		// Determine winding at three points in the chain. If a winding with sufficient quality is found, use it. Otherwise use the winding with the best quality.
		let offsets = [0.5, 0.25, 0.75];
		let winding = { winding: 0, quality: -1 };
		// Don't go too close to segments, to avoid special winding cases:
		let tMin = 1e-3;
		let tMax = 1 - tMin;
		// for (let i = 0; i < offsets.length; i++) {
		for _ in 0..offsets.len() {
			if winding.quality >= 0.5 {
				break;
			}
			let length = totalLength * offsets[i];
			// for (let j = 0; j < chain.length; j++) {
			for entry in chain {
				let curveLength = entry.length;
				if (length <= curveLength) {
					let curve = entry.curve;
					let path = curve._path;
					let parent = path._parent;
					let operand = if parent instanceof CompoundPath { parent } else { path };
					let t = Numerical.clamp(curve.getTimeAt(length), tMin, tMax);
					let pt = curve.getPointAtTime(t);
					// Determine the direction in which to check the winding
					// from the point (horizontal or vertical), based on the
					// curve's direction at that point. If tangent is less
					// than 45°, cast the ray vertically, else horizontally.
					let dir = abs(curve.getTangentAtTime(t).y) < Math.SQRT1_2;
					// While subtracting, we need to omit this curve if it is contributing to the second operand and is outside the first operand.
					let wind = if operation == Operator::Subtract && path2 {
						// Calculate path winding at point depending on operand.
						let otherPath = if operand == path1 { path2 } else { path1 };
						let pathWinding = otherPath._getWinding(pt, dir, true);
						// Check if curve should be omitted.
						if operand == path1 && pathWinding.winding || operand == path2 && !pathWinding.winding {
							// Check if quality is not good enough...
							if pathWinding.quality < 1 {
								// ...and if so, skip this point...
								continue;
							} else {
								// ...otherwise, omit this curve.
								{ winding: 0, quality: 1 }
							}
						}
					} else {
						let curves = curveCollisionsMap[path._id][curve.getIndex()];

						// `curves` can either be an array of curves, or an object containing of
						// the form `{ hor: [], ver: [] }` (see `curveCollisionsMap`), with each
						// key / value pair holding only those curves that can be crossed by a
						// horizontal / vertical line through the point to be checked.
						let curvesList = if dir { curves["hor"] } else { curves["ver"] };
						
						wind || getWinding(pt, curvesList, dir, true);
					};
					if wind.quality > winding.quality {
						winding = wind;
					}
					break;
				}
				length -= curveLength;
			}
		}
		// Now assign the winding to the entire curve chain.
		// for (let j = chain.length - 1; j >= 0; j--) {
		for chainEntry in chain.iter().rev() {
			chainEntry.segment._winding = winding;
		}
	}

	/// Private method to trace closed paths from a list of segments, according
	/// to a the their winding number contribution and a custom operator.
	///
	/// @param {Segment[]} segments array of segments to trace closed paths
	/// @param {Function} operator the operator lookup table that receives as key
	///     the winding number contribution of a curve and returns a boolean
	///     value indicating whether the curve should be included in result
	/// @return {Path[]} the traced closed paths
	fn tracePaths(segments, operation: Operator) {
		let paths = [];
		let mut starts;

		fn isValid(seg) {
			return !!(seg
				&& !seg._visited
				&& {
					let winding = seg._winding;
					let key: i32 = (winding || {}).winding;
					// Unite operations need special handling of segments with a winding contribution of two (part of both areas), which are only valid if they are part of the result's contour, not contained inside another area.
					operators_lookup_table(operation, key) && !(
						operation == Operator::Unite && winding.winding == 2
						// No contour if both windings are non-zero.
						&& winding.windingL && winding.windingR)
					}
				);
		}

		fn isStart(seg) {
			if seg {
				// for (let i = 0; i < starts.length; i++) {
				for start in starts {
					if seg == start {
						return true;
					}
				}
			}
			return false;
		}

		fn visitPath(path) {
			let segments = path._segments;
			// for (let i = 0; i < segments.length; i++) {
			for segment in segments {
				segment._visited = true;
			}
		}

		// If there are multiple possible intersections, find the ones that's
		// either connecting back to start or are not visited yet, and will be
		// part of the boolean result:
		fn getCrossingSegments(segment, collectStarts) {
			let inter = segment._intersection;
			let start = inter;
			let crossings = [];
			if collectStarts {
				starts = [segment];
			}

			fn collect(inter, end) {
				while (inter && inter != end) {
					let other = inter._segment;
					let path = other && other._path;
					if path {
						let next = other.getNext() || path.getFirstSegment();
						let nextInter = next._intersection;
						// See if this segment and the next are not visited yet,
						// or are bringing us back to the start, and are both
						// valid, meaning they're part of the boolean result.
						if (other != segment
							&& (isStart(other)
								|| isStart(next)
								|| next && (
									isValid(other)
									&& (isValid(next)
										// If next segment isn't valid, its intersection
										// to which we may switch may be, so check that.
										|| nextInter
										&& isValid(nextInter._segment)
									)
								)
							)
						) {
							crossings.push(other);
						}
						if collectStarts {
							starts.push(other);
						}
					}
					inter = inter._next;
				}
			}

			if inter {
				collect(inter);
				// Find the beginning of the linked intersections and loop all
				// the way back to start, to collect all valid intersections.
				while inter && inter._previous {
					inter = inter._previous;
				}
				collect(inter, start);
			}
			return crossings;
		}

		// Sort segments to give non-ambiguous segments the preference as
		// starting points when tracing: prefer segments with no intersections
		// over intersections, and process intersections with overlaps last:
		segments.sort_by(|seg1, seg2| {
			let inter1 = seg1._intersection;
			let inter2 = seg2._intersection;
			let over1 = !!(inter1 && inter1._overlap);
			let over2 = !!(inter2 && inter2._overlap);
			let path1 = seg1._path;
			let path2 = seg2._path;
			// Use bitwise-or to sort cases where only one segment is an overlap
			// or intersection separately, and fall back on natural order within the path.
			if over1 ^ over2 {
				if over1 { 1 } else { -1 }
				// NOTE: inter1 & 2 are objects, convert to boolean first
				// as otherwise toString() is called on them.
			} else {
				if !inter1 ^ !inter2 {
					if inter1 { 1 } else { -1 }
				} else {
					// All other segments, also when comparing two overlaps
					// or two intersections, are sorted by their order.
					// Sort by path id to group segments on the same path.
					if path1 != path2 { path1._id - path2._id } else { seg1._index - seg2._index }
				}
			}
		});

		// for (let i = 0; i < segments.length; i++) {
		// 	let seg = segments[i];
		for seg in segments {
			let valid = isValid(seg);
			let path = null;
			let finished = false;
			let closed = true;
			let branches = [];
			let branch;
			let visited;
			let handleIn;
			// If all encountered segments in a path are overlaps, we may have two fully overlapping paths that need special handling.
			if (valid && seg._path._overlapsOnly) {
				// TODO: Don't we also need to check for multiple overlaps?
				let path1 = seg._path;
				let path2 = seg._intersection._segment._path;
				if path1.compare(path2) {
					// Only add the path to the result if it has an area.
					if path1.getArea() {
						paths.push(path1.clone(false));
					}
					// Now mark all involved segments as visited.
					visitPath(path1);
					visitPath(path2);
					valid = false;
				}
			}
			// Do not start with invalid segments (segments that were already visited, or that are not going to be part of the result).
			while valid {
				// For each segment we encounter, see if there are multiple
				// crossings, and if so, pick the best one:
				let first = !path;
				let crossings = getCrossingSegments(seg, first);
					// Get the other segment of the first found crossing.
				let other = crossings.shift();
				let finished = !first && (isStart(seg) || isStart(other));
				let cross = !finished && other;
				if first {
					path = new Path(PathItem.NO_INSERT);
					// Clear branch to start a new one with each new path.
					branch = null;
				}
				if finished {
					// If we end up on the first or last segment of an operand,
					// copy over its closed state, to support mixed open/closed
					// scenarios as described in #1036
					if seg.isFirst() || seg.isLast() {
						closed = seg._path._closed;
					}
					seg._visited = true;
					break;
				}
				if cross && branch {
					// If we're about to cross, start a new branch and add the
					// current one to the list of branches.
					branches.push(branch);
					branch = null;
				}
				if !branch {
					// Add the branch's root segment as the last segment to try,
					// to see if we get to a solution without crossing.
					if cross {
						crossings.push(seg);
					}
					branch = {
						start: path._segments.length,
						crossings: crossings,
						visited: visited = [],
						handleIn: handleIn
					};
				}
				if cross {
					seg = other;
				}
				// If an invalid segment is encountered, go back to the last
				// crossing and try other possible crossings, as well as not
				// crossing at the branch's root.
				if !isValid(seg) {
					// Remove the already added segments, and mark them as not
					// visited so they become available again as options.
					path.removeSegments(branch.start);
					// for (let j = 0; j < visited.length; j++) {
					// 	visited[j]._visited = false;
					for visitedEntry in visited {
						visitedEntry._visited = false;
					}
					visited.length = 0;
					// Go back to the branch's root segment where the crossing
					// happened, and try other crossings. Note that this also
					// tests the root segment without crossing as it is added to
					// the list of crossings when the branch is created above.
					loop {
						seg = branch && branch.crossings.shift();
						if (!seg || !seg._path) {
							seg = null;
							// If there are no segments left, try previous
							// branches until we find one that works.
							branch = branches.pop();
							if (branch) {
								visited = branch.visited;
								handleIn = branch.handleIn;
							}
						}

						if !(branch && !isValid(seg)) {
							break;
						}
					}
					if !seg {
						break;
					}
				}
				// Add the segment to the path, and mark it as visited.
				// But first we need to look ahead. If we encounter the end of
				// an open path, we need to treat it the same way as the fill of
				// an open path would: Connecting the last and first segment
				// with a straight line, ignoring the handles.
				let next = seg.getNext();
				path.add(new Segment(seg._point, handleIn, next && seg._handleOut));
				seg._visited = true;
				visited.push(seg);
				// If this is the end of an open path, go back to its first segment but ignore its handleIn (see above for handleOut).
				seg = next || seg._path.getFirstSegment();
				handleIn = next && next._handleIn;
			}
			if finished {
				if closed {
					// Carry over the last handleIn to the first segment.
					path.getFirstSegment().setHandleIn(handleIn);
					path.setClosed(closed);
				}
				// Only add finished paths that cover an area to the result.
				if path.getArea() != 0 {
					paths.push(path);
				}
			}
		}
		return paths;
	}


	/// Returns the winding contribution number of the given point in respect
	/// to this PathItem.
	///
	/// @param {Point} point the location for which to determine the winding
	///     contribution
	/// @param {Number} [dir=0] the direction in which to determine the
	///     winding contribution, `0`: in x-direction, `1`: in y-direction
	/// @return {Object} an object containing the calculated winding number, as
	///     well as an indication whether the point was situated on the contour
	pub fn _getWinding(&self, point: Point, dir: i32, closed: bool) -> Option<WindingInfo> {
		return getWinding(point, this.getCurves(), dir, closed);
	}

	/// {@grouptitle Boolean Path Operations}
	///
	/// Unites the geometry of the specified path with this path's geometry
	/// and returns the result as a new path item.
	///
	/// @option [options.insert=true] {Boolean} whether the resulting item
	///     should be inserted back into the scene graph, above both paths
	///     involved in the operation
	///
	/// @param {PathItem} path the path to unite with
	/// @param {Object} [options] the boolean operation options
	/// @return {PathItem} the resulting path item
	pub fn unite(path: PathItem, options: BooleanOperationOptions) {
		return traceBoolean(this, path, Operator::Unite, options);
	}

	/// Intersects the geometry of the specified path with this path's
	/// geometry and returns the result as a new path item.
	///
	/// @option [options.insert=true] {Boolean} whether the resulting item
	///     should be inserted back into the scene graph, above both paths
	///     involved in the operation
	/// @option [options.trace=true] {Boolean} whether the tracing method is
	///     used, treating both paths as areas when determining which parts
	///     of the paths are to be kept in the result, or whether the first
	///     path is only to be split at intersections, keeping the parts of
	///     the curves that intersect with the area of the second path.
	///
	/// @param {PathItem} path the path to intersect with
	/// @param {Object} [options] the boolean operation options
	/// @return {PathItem} the resulting path item
	pub fn intersect(path: PathItem, options) {
		return traceBoolean(this, path, Operator::Intersect, options);
	}

	/// Subtracts the geometry of the specified path from this path's
	/// geometry and returns the result as a new path item.
	///
	/// @option [options.insert=true] {Boolean} whether the resulting item
	///     should be inserted back into the scene graph, above both paths
	///     involved in the operation
	/// @option [options.trace=true] {Boolean} whether the tracing method is
	///     used, treating both paths as areas when determining which parts
	///     of the paths are to be kept in the result, or whether the first
	///     path is only to be split at intersections, removing the parts of
	///     the curves that intersect with the area of the second path.
	///
	/// @param {PathItem} path the path to subtract
	/// @param {Object} [options] the boolean operation options
	/// @return {PathItem} the resulting path item
	pub fn subtract(path: PathItem, options) {
		return traceBoolean(this, path, Operator::Subtract, options);
	}

	/// Excludes the intersection of the geometry of the specified path with
	/// this path's geometry and returns the result as a new path item.
	///
	/// @option [options.insert=true] {Boolean} whether the resulting item
	///     should be inserted back into the scene graph, above both paths
	///     involved in the operation
	///
	/// @param {PathItem} path the path to exclude the intersection of
	/// @param {Object} [options] the boolean operation options
	/// @return {PathItem} the resulting path item
	pub fn exclude(path: PathItem, options) {
		return traceBoolean(this, path, Operator::Exclude, options);
	}

	/// Splits the geometry of this path along the geometry of the specified
	/// path returns the result as a new group item. This is equivalent to
	/// calling {@link #subtract(path)} and {@link #intersect(path)} and
	/// putting the results into a new group.
	///
	/// @option [options.insert=true] {Boolean} whether the resulting item
	///     should be inserted back into the scene graph, above both paths
	///     involved in the operation
	/// @option [options.trace=true] {Boolean} whether the tracing method is
	///     used, treating both paths as areas when determining which parts
	///     of the paths are to be kept in the result, or whether the first
	///     path is only to be split at intersections.
	///
	/// @param {PathItem} path the path to divide by
	/// @param {Object} [options] the boolean operation options
	/// @return {PathItem} the resulting path item
	pub fn divide(path: PathItem, options) {
		if options && (options.trace == false || options.stroke) {
			splitBoolean(this, path, Operator::Divide)
		} else {
			createResult(
				[this.subtract(path, options), this.intersect(path, options)],
				true,
				this,
				path,
				options
			)
		}
	}

	/// Resolves all crossings of a path item by splitting the path or
	/// compound-path in each self-intersection and tracing the result.
	/// If possible, the existing path / compound-path is modified if the
	/// amount of resulting paths allows so, otherwise a new path /
	/// compound-path is created, replacing the current one.
	///
	/// @return {PathItem} the resulting path item
	pub fn resolveCrossings() {
		let children = this._children;
		// Support both path and compound-path items
		let paths = children || [this];

		fn hasOverlap(seg, path) {
			let inter = seg && seg._intersection;
			return inter && inter._overlap && inter._path == path;
		}

		// First collect all overlaps and crossings while taking note of the
		// existence of both.
		let hasOverlaps = false;
		let hasCrossings = false;
		let intersections = this.getIntersections(null, |inter| inter.hasOverlap() && (hasOverlaps = true) || inter.isCrossing() && (hasCrossings = true));
		// We only need to keep track of curves that need clearing outside of divideLocations() if two calls are necessary.
		let clearCurves = hasOverlaps && hasCrossings && [];
		intersections = CurveLocation.expand(intersections);
		if (hasOverlaps) {
			// First divide in all overlaps, and then remove the inside of
			// the resulting overlap ranges.
			let overlaps = divideLocations(intersections, |inter| inter.hasOverlap(), clearCurves);
			// for (let i = overlaps.length - 1; i >= 0; i--) {
				// let overlap = overlaps[i];
			for overlap in overlaps.iter().rev() {
				let path = overlap._path;
				let seg = overlap._segment;
				let prev = seg.getPrevious();
				let next = seg.getNext();
				if hasOverlap(prev, path) && hasOverlap(next, path) {
					seg.remove();
					prev._handleOut._set(0, 0);
					next._handleIn._set(0, 0);
					// If the curve that is left has no length, remove it altogether. Check for paths with only one segment before removal, since `prev.getCurve() == null`.
					if prev != seg && !prev.getCurve().hasLength() {
						// Transfer handleIn when removing segment:
						next._handleIn.set(prev._handleIn);
						prev.remove();
					}
				}
			}
		}
		if (hasCrossings) {
			// Divide any remaining intersections that are still part of valid paths after the removal of overlaps.
			divideLocations(intersections, hasOverlaps && |inter| {
				// Check both involved curves to see if they're still valid,
				// meaning they are still part of their paths.
				let curve1 = inter.getCurve();
				let seg1 = inter.getSegment();
				// Do not call getCurve() and getSegment() on the other
				// intersection yet, as it too is in the intersections
				// array and will be divided later. But check if its
				// current curve is valid, as required by some rare edge
				// cases, related to intersections on the same curve.
				let other = inter._intersection;
				let curve2 = other._curve;
				let seg2 = other._segment;
				if curve1 && curve2 && curve1._path && curve2._path {
					return true;
				}
				// Remove all intersections that were involved in the
				// handling of overlaps, to not confuse tracePaths().
				if seg1 {
					seg1._intersection = null;
				}
				if seg2 {
					seg2._intersection = null;
				}
			}, clearCurves);
			if clearCurves {
				clearCurveHandles(clearCurves);
			}
			// Finally resolve self-intersections through tracePaths()
			paths = tracePaths(Base.each(paths, |path| {
				Base.push(this, path._segments);
			}, []));
		}
		// Determine how to return the paths: First try to recycle the
		// current path / compound-path, if the amount of paths does not
		// require a conversion.
		let length = paths.length;
		if (length > 1 && children) {
			if (paths != children) {
				this.setChildren(paths);
			}
		} else if (length == 1 && !children) {
			if (paths[0] != this) {
				this.setSegments(paths[0].removeSegments());
			}
		}
		let item = this;
		// Otherwise create a new compound-path and see if we can reduce it,
		// and attempt to replace this item with it.
		if (!item) {
			item = new CompoundPath(PathItem.NO_INSERT);
			item.addChildren(paths);
			item = item.reduce();
			item.copyAttributes(this);
			this.replaceWith(item);
		}
		return item;
	}

	/// Fixes the orientation of the sub-paths of a compound-path, assuming
	/// that non of its sub-paths intersect, by reorienting them so that they
	/// are of different winding direction than their containing paths,
	/// except for disjoint sub-paths, i.e. islands, which are oriented so
	/// that they have the same winding direction as the the biggest path.
	///
	/// @param {Boolean} [nonZero=false] controls if the non-zero fill-rule
	///     is to be applied, by counting the winding of each nested path and
	///     discarding sub-paths that do not contribute to the final result
	/// @param {Boolean} [clockwise] if provided, the orientation of the root
	///     paths will be set to the orientation specified by `clockwise`,
	///     otherwise the orientation of the largest root child is used.
	/// @return {PathItem} a reference to the item itself, reoriented
	pub fn reorient(nonZero: bool, clockwise: bool) {
		let children = this._children;
		if (children && children.length) {
			this.setChildren(
				reorientPaths(this.removeChildren(),
					// Handle both even-odd and non-zero rule.
					|w| !!(if nonZero { w } else { w & 1 }),
					clockwise
				)
			);
		} else if (clockwise != undefined) {
			this.setClockwise(clockwise);
		}
		return this;
	}
}

impl PathItem {
	/// Returns a point that is guaranteed to be inside the path.
	///
	/// @bean
	/// @type Point
	pub fn getInteriorPoint(&self) -> Point {
		let bounds = self.getBounds();
		let point = bounds.getCenter(true);
		if !self.contains(point) {
			// Since there is no guarantee that a poly-bezier path contains
			// the center of its bounding rectangle, we shoot a ray in x
			// direction and select a point between the first consecutive
			// intersections of the ray on the left.
			let curves = self.getCurves();
			let y = point.y;
			let intercepts = [];
			let roots = [];
			// Process all y-monotone curves that intersect the ray at y:
			// for (let i = 0; i < curves.length; i++) {
			for curve in curves {
				let v = curve.getValues();
				let o0 = v[1];
				let o1 = v[3];
				let o2 = v[5];
				let o3 = v[7];
				if y >= min(o0, o1, o2, o3) && y <= max(o0, o1, o2, o3) {
					let monoCurves = Curve.getMonoCurves(v);
					// for (let j = 0; j < monoCurves.length; j++) {
					for mv in monoCurves {
						let mo0 = mv[1];
						let mo3 = mv[7];
						// Only handle curves that are not horizontal and that can cross the point's ordinate.
						if (mo0 != mo3) && (y >= mo0 && y <= mo3 || y >= mo3 && y <= mo0) {
							let x = if y == mo0 {
								mv[0]
							} else {
								if y == mo3 {
									mv[6]
								} else {
									if Curve.solveCubic(mv, 1, y, roots, 0, 1) == 1 {
										Curve.getPoint(mv, roots[0]).x
									} else {
										(mv[0] + mv[6]) / 2
									}
								}
							};
							intercepts.push(x);
						}
					}
				}
			}
			if intercepts.length > 1 {
				intercepts.sort_by(|a, b| a - b);
				point.x = (intercepts[0] + intercepts[1]) / 2;
			}
		}
		return point;
	}
}
