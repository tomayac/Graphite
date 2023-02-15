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

use super::boolean_ops::PathItem;

pub struct CollisionDetection;

impl CollisionDetection {
	/// Finds collisions between axis aligned bounding boxes of items.
	///
	/// This function takes the bounds of all items in the items1 and items2
	/// arrays and calls findBoundsCollisions().
	///
	/// @param {Array} items1 Array of items for which collisions should be
	///     found.
	/// @param {Array} [items2] Array of items  that the first array should be
	///     compared with. If not provided, collisions between items within
	///     the first array will be returned.
	/// @param {Number} [tolerance] If provided, the tolerance will be added to
	///     all sides of each bounds when checking for collisions.
	/// @returns {Array} Array containing for the bounds at the same index in
	///     items1 an array of the indexes of colliding bounds in items2
	pub fn findItemBoundsCollisions(&self, items1: &[PathItem], items2: &[PathItem], tolerance: Option<f64>) -> Vec<Vec<usize>> {
		fn getBounds(items: &[PathItem]) -> Vec<[f64; 4]> {
			// var bounds = new Array(items.length);
			// for (var i = 0; i < items.length; i++) {
			// 	var rect = items[i].getBounds();
			// 	bounds[i] = [rect.left, rect.top, rect.right, rect.bottom];
			// }
			items
				.iter()
				.map(|item| {
					let rect = item.getBounds();
					[rect.left, rect.top, rect.right, rect.bottom]
				})
				.collect::<Vec<_>>()
		}

		let bounds1 = getBounds(items1);
		let bounds2 = if items2.is_empty() || items2.iter().eq(items1.iter()) { bounds1 } else { getBounds(items2) };
		self.findBoundsCollisions(bounds1.as_slice(), bounds2.as_slice(), tolerance, false, false)
	}

	/// Finds collisions between curves bounds. For performance reasons this
	/// uses broad bounds of the curve, which can be calculated much faster than
	/// the actual bounds. Broad bounds guarantee to contain the full curve,
	/// but they are usually larger than the actual bounds of a curve.
	///
	/// This function takes the broad bounds of all curve values in the curves1
	/// and curves2 arrays and calls findBoundsCollisions().
	///
	/// @param {Array} curves1 Array of curve values for which collisions should be found.
	/// @param {Array} [curves2] Array of curve values that the first array
	///     should be compared with. If not provided, collisions between curve
	///     bounds within the first arrray will be returned.
	/// @param {Number} [tolerance] If provided, the tolerance will be added to
	///     all sides of each bounds when checking for collisions.
	/// @param {Boolean} [bothAxis] If true, the sweep is performed along both
	///     axis, and the results include collisions for both: `{ hor, ver }`.
	/// @returns {Array} Array containing for the bounds at the same index in
	///     curves1 an array of the indexes of colliding bounds in curves2
	pub fn findCurveBoundsCollisions(&self, curves1: &[[f64; 8]], curves2: &[[f64; 8]], tolerance: Option<f64>) -> Vec<Vec<usize>> {
		let bounds1 = Self::getBounds(curves1);
		let bounds2 = if curves2.is_empty() || curves2 == curves1 { bounds1 } else { Self::getBounds(curves2) };

		self.findBoundsCollisions(bounds1.as_slice(), bounds2.as_slice(), tolerance, false, false)
	}

	pub fn findCurveBoundsCollisionsBothAxis(&self, curves1: &[[f64; 8]], curves2: &[[f64; 8]], tolerance: Option<f64>) -> Vec<(Vec<usize>, Vec<usize>)> {
		let bounds1 = Self::getBounds(curves1);
		let bounds2 = if curves2.is_empty() || curves2 == curves1 { bounds1 } else { Self::getBounds(curves2) };

		let hor = self.findBoundsCollisions(bounds1.as_slice(), bounds2.as_slice(), tolerance, false, true);
		let ver = self.findBoundsCollisions(bounds1.as_slice(), bounds2.as_slice(), tolerance, true, true);
		// let list = [];
		// for (var i = 0; i < hor.length; i++) {
		//     list[i] = (hor[i], ver[i]);
		// }
		hor.into_iter().zip(ver.into_iter()).collect::<Vec<_>>()
	}

	fn getBounds(curves: &[[f64; 8]]) -> Vec<[f64; 4]> {
		// let bounds = new Array(curves.length);
		// for (var i = 0; i < curves.length; i++) {
		//     var v = curves[i];
		//     bounds[i] = [
		//         min(v[0], v[2], v[4], v[6]),
		//         min(v[1], v[3], v[5], v[7]),
		//         max(v[0], v[2], v[4], v[6]),
		//         max(v[1], v[3], v[5], v[7])
		//     ];
		// }
		curves
			.iter()
			.map(|v| {
				[
					v[0].min(v[2]).min(v[4]).min(v[6]),
					v[1].min(v[3]).min(v[5]).min(v[7]),
					v[0].max(v[2]).max(v[4]).max(v[6]),
					v[1].max(v[3]).max(v[5]).max(v[7]),
				]
			})
			.collect::<Vec<_>>()
	}

	/// Finds collisions between two sets of bounding rectangles.
	///
	/// The collision detection is implemented as a sweep and prune algorithm
	/// with sweep either along the x or y axis (primary axis) and immediate
	/// check on secondary axis for potential pairs.
	///
	/// Each entry in the bounds arrays must be an array of length 4 with
	/// x0, y0, x1, and y1 as the array elements.
	///
	/// The returned array has the same length as bounds1. Each entry
	/// contains an array with all indices of overlapping bounds of
	/// bounds2 (or bounds1 if bounds2 is not provided) sorted
	/// in ascending order.
	///
	/// If the second bounds array parameter is null, collisions between bounds
	/// within the first bounds array will be found. In this case the indexed
	/// returned for each bounds will not contain the bounds' own index.
	///
	///
	/// @param {Array} boundsA Array of bounds objects for which collisions
	///     should be found.
	/// @param {Array} [boundsB] Array of bounds that the first array should
	///     be compared with. If not provided, collisions between bounds within
	///     the first arrray will be returned.
	/// @param {Number} [tolerance] If provided, the tolerance will be added to
	///     all sides of each bounds when checking for collisions.
	/// @param {Boolean} [sweepVertical] If true, the sweep is performed along
	///     the y-axis.
	/// @param {Boolean} [onlySweepAxisCollisions] If true, no collision checks
	///     will be done on the secondary axis.
	/// @returns {Array} Array containing for the bounds at the same index in
	///     boundsA an array of the indexes of colliding bounds in boundsB
	pub fn findBoundsCollisions(&self, boundsA: &[[f64; 4]], boundsB: &[[f64; 4]], tolerance: Option<f64>, sweepVertical: bool, onlySweepAxisCollisions: bool) -> Vec<Vec<usize>> {
		let this = boundsB.is_empty() || boundsA == boundsB;
		let allBounds = if this {
			boundsA.iter().collect::<Vec<_>>().as_slice()
		} else {
			boundsA.iter().chain(boundsB.iter()).collect::<Vec<_>>().as_slice()
		};
		let lengthA = boundsA.len();
		let lengthAll = allBounds.len();

		// Binary search utility function.
		// For multiple same entries, this returns the rightmost entry.
		// https://en.wikipedia.org/wiki/Binary_search_algorithm#Procedure_for_finding_the_rightmost_element
		let binarySearch = |indices: &[usize], coord, value| {
			let lo = 0;
			let hi = indices.len();
			while lo < hi {
				let mid = ((hi as f64 + lo as f64) / 2.) as usize;
				if allBounds[indices[mid]][coord] < value {
					lo = mid + 1;
				} else {
					hi = mid;
				}
			}
			return lo - 1;
		};

		// Set coordinates for primary and secondary axis depending on sweep
		// direction. By default we sweep in horizontal direction, which
		// means x is the primary axis.
		let pri0 = if sweepVertical { 1 } else { 0 };
		let pri1 = pri0 + 2;
		let sec0 = if sweepVertical { 0 } else { 1 };
		let sec1 = sec0 + 2;
		// Create array with all indices sorted by lower boundary on primary axis.
		// let allIndicesByPri0 = new Array(lengthAll);
		// for (var i = 0; i < lengthAll; i++) {
		// 	allIndicesByPri0[i] = i;
		// }
		let allIndicesByPri0 = (0..lengthAll).collect::<Vec<_>>();
		// allIndicesByPri0.sort_by(|&i1, &i2| allBounds[i1][pri0] - allBounds[i2][pri0]);
		allIndicesByPri0.sort_by(|&i1, &i2| {
			let a = allBounds[i1][pri0];
			let b = allBounds[i2][pri0];
			if a < b {
				std::cmp::Ordering::Less
			} else if a > b {
				std::cmp::Ordering::Greater
			} else {
				std::cmp::Ordering::Equal
			}
		});
		// Sweep along primary axis. Indices of active bounds are kept in an array sorted by higher boundary on primary axis.
		let mut activeIndicesByPri1 = Vec::<usize>::new();
		let mut allCollisions: Vec<Vec<usize>> = Vec::with_capacity(lengthA);
		// for (var i = 0; i < lengthAll; i++) {
		for i in 0..lengthAll {
			let curIndex = allIndicesByPri0[i];
			let curBounds = allBounds[curIndex];
			// The original index in boundsA or boundsB:
			let origIndex = if this { curIndex } else { curIndex - lengthA };
			let isCurrentA = curIndex < lengthA;
			let isCurrentB = this || !isCurrentA;
			let mut curCollisions = Vec::<usize>::new();
			if activeIndicesByPri1.len() > 0 {
				// remove (prune) indices that are no longer active.
				let pruneCount = binarySearch(activeIndicesByPri1.as_slice(), pri1, curBounds[pri0] - tolerance.unwrap_or_default()) + 1;
				// activeIndicesByPri1.splice(0, pruneCount);
				activeIndicesByPri1.drain(pruneCount..);
				// Add collisions for current index.
				if this && onlySweepAxisCollisions {
					// All active indexes can be added, no further checks needed
					curCollisions.extend(activeIndicesByPri1);
					// Add current index to collisions of all active indexes
					// for (var j = 0; j < activeIndicesByPri1.length; j++) {
					// 	var activeIndex = activeIndicesByPri1[j];
					// 	allCollisions[activeIndex].push(origIndex);
					// }
					for activeIndex in activeIndicesByPri1 {
						allCollisions[activeIndex].push(origIndex);
					}
				} else {
					let curSec1 = curBounds[sec1];
					let curSec0 = curBounds[sec0];
					// for (var j = 0; j < activeIndicesByPri1.length; j++) {
					// let activeIndex = activeIndicesByPri1[j];
					for activeIndex in activeIndicesByPri1 {
						let activeBounds = allBounds[activeIndex];
						let isActiveA = activeIndex < lengthA;
						let isActiveB = this || activeIndex >= lengthA;

						// Check secondary axis bounds if necessary.
						if onlySweepAxisCollisions
							|| (isCurrentA && isActiveB || isCurrentB && isActiveA)
								&& (curSec1 >= activeBounds[sec0] - tolerance.unwrap_or_default() && curSec0 <= activeBounds[sec1] + tolerance.unwrap_or_default())
						{
							// Add current index to collisions of active
							// indices and vice versa.
							if isCurrentA && isActiveB {
								curCollisions.push(if this { activeIndex } else { activeIndex - lengthA });
							}
							if isCurrentB && isActiveA {
								allCollisions[activeIndex].push(origIndex);
							}
						}
					}
				}
			}
			if isCurrentA {
				if boundsA == boundsB {
					// If both arrays are the same, add self collision.
					curCollisions.push(curIndex);
				}
				// Add collisions for current index.
				allCollisions[curIndex] = curCollisions;
			}
			// Add current index to active indices. Keep array sorted by
			// their higher boundary on the primary axis.s
			if activeIndicesByPri1.len() > 0 {
				let curPri1 = curBounds[pri1];
				let index = binarySearch(activeIndicesByPri1.as_slice(), pri1, curPri1);
				// activeIndicesByPri1.splice(index + 1, 0, curIndex);
				activeIndicesByPri1.insert(index, curIndex);
			} else {
				activeIndicesByPri1.push(curIndex);
			}
		}
		// Sort collision indices in ascending order.
		// for (var i = 0; i < allCollisions.length; i++) {
		// 	let collisions = allCollisions[i];
		// 	// if collisions {
		// 		collisions.sort_by(|i1, i2| i1 - i2);
		// 	// }
		// }
		for collisions in allCollisions {
			// collisions.sort_by(|i1, i2| i1 - i2);
			collisions.sort_by(|i1, i2| {
				if i1 < i2 {
					std::cmp::Ordering::Less
				} else if i1 > i2 {
					std::cmp::Ordering::Greater
				} else {
					std::cmp::Ordering::Equal
				}
			})
		}
		return allCollisions;
	}
}
