import { invoke } from "@tauri-apps/api";
import JSONBig from 'json-bigint'
import * as commands from "./bindings";

/** @internal */
export const safeJSON = JSONBig({ useNativeBigInt: true })


import { createSubscriptionRouter, type SubscriptionRouter } from "@/wasm-communication/subscription-router";
export const subscriptions: SubscriptionRouter = createSubscriptionRouter();


async function dispatch_response(response: [any]): Promise<void>;


/* tslint:disable */
/* eslint-disable */
export class TauriHandle {
  new(frontend_message_handler_callback: Function) {}
/**
* @param {string} platform
*/
  async initAfterFrontendReady(platform: string) {
	dispatch_response(await commands.init_after_frontend_ready( platform ));
  }
/**
* Displays a dialog with an error message
* @param {string} title
* @param {string} description
*/
  async errorDialog(title: string, description: string) {
	dispatch_response(await commands.error_dialog( title, description ));
  }
/**
* @param {string} preferences
/**
/**
* Answer whether or not the editor is in development mode
* @returns {boolean}
*/
async inDevelopmentMode(): Promise<boolean> {
  return await invoke("in_development_mode");
}
/**
* Get the constant `FILE_SAVE_SUFFIX`
* @returns {string}
*/
async fileSaveSuffix(): Promise<string> {
  return await invoke("file_save_suffix");
}
/**
* Get the constant `GRAPHITE_DOCUMENT_VERSION`
* @returns {string}
*/
async graphiteDocumentVersion(): Promise<string> {
  return await invoke("graphite_document_version");
}
/**
* Update layout of a given UI
* @param {any} layout_target
* @param {bigint} widget_id
* @param {any} value
*/
async updateLayout(layout_target: any, widget_id: bigint, value: any) {
  let nvalue = value;
  if (!nvalue) { nvalue = "unknown" }
  console.log(nvalue)
  dispatch_response(await commands.update_layout( layout_target,  widget_id, nvalue ));
}
/**
* @param {string} preferences
*/
async loadPreferences(preferences: string) {
  dispatch_response(await commands.load_preferences( preferences ));
}
/**
* @param {bigint} document_id
*/
async selectDocument(document_id: bigint) {
  dispatch_response(await commands.select_document( document_id ));
}
/**
*/
async newDocumentDialog() {
  dispatch_response(await invoke("new_document_dialog"));
}
/**
*/
async documentOpen() {
  dispatch_response(await invoke("document_open"));
}
/**
* @param {string} document_name
* @param {string} document_serialized_content
*/
async openDocumentFile(document_name: string, document_serialized_content: string) {
  dispatch_response(await commands.open_document_file( document_name,  document_serialized_content ));
}
/**
* @param {bigint} document_id
* @param {string} document_name
* @param {boolean} document_is_saved
* @param {string} document_serialized_content
*/
async openAutoSavedDocument(document_id: bigint, document_name: string, document_is_saved: Promise<boolean>, document_serialized_content: string) {
  dispatch_response(await commands.open_auto_saved_document( document_id,  document_name,  document_is_saved,  document_serialized_content ));
}
/**
* @param {bigint} document_id
*/
async triggerAutoSave(document_id: bigint) {
  dispatch_response(await commands.trigger_auto_save( document_id ));
}
/**
* @param {bigint} document_id
*/
async closeDocumentWithConfirmation(document_id: bigint) {
  dispatch_response(await commands.close_document_with_confirmation( document_id ));
}
/**
* @param {string} localized_commit_date
*/
async requestAboutGraphiteDialogWithLocalizedCommitDate(localized_commit_date: string) {
  dispatch_response(await commands.request_about_graphite_dialog_with_localized_commit_date( localized_commit_date ));
}
/**
* Send new bounds when document panel viewports get resized or moved within the editor
* [left, top, right, bottom]...
* @param {Float64Array} bounds_of_viewports
*/
async boundsOfViewports(bounds_of_viewports: Float64Array) {
  dispatch_response(await commands.bounds_of_viewports( bounds_of_viewports ));
}
/**
* Mouse movement within the screenspace bounds of the viewport
* @param {number} x
* @param {number} y
* @param {number} mouse_keys
* @param {number} modifiers
*/
  async onMouseMove(x: number, y: number, mouse_keys: number, modifiers: number) {
	dispatch_response(await commands.on_mouse_move( x, y,  mouse_keys, modifiers));
  }
/**
* Mouse scrolling within the screenspace bounds of the viewport
* @param {number} x
* @param {number} y
* @param {number} mouse_keys
* @param {number} wheel_delta_x
* @param {number} wheel_delta_y
* @param {number} wheel_delta_z
* @param {number} modifiers
*/
  async onWheelScroll(x: number, y: number, mouse_keys: number, wheel_delta_x: number, wheel_delta_y: number, wheel_delta_z: number, modifiers: number) {
	dispatch_response(await commands.on_wheel_scroll( x, y,  mouse_keys, wheel_delta_x, wheel_delta_y,  wheel_delta_z, modifiers));
  }
/**
* A mouse button depressed within screenspace the bounds of the viewport
* @param {number} x
* @param {number} y
* @param {number} mouse_keys
* @param {number} modifiers
*/
  async onMouseDown(x: number, y: number, mouse_keys: number, modifiers: number) {
	dispatch_response(await commands.on_mouse_down( x, y,  mouse_keys, modifiers));
  }
/**
* A mouse button released
* @param {number} x
* @param {number} y
* @param {number} mouse_keys
* @param {number} modifiers
*/
  async onMouseUp(x: number, y: number, mouse_keys: number, modifiers: number) {
	dispatch_response(await commands.on_mouse_up( x, y,  mouse_keys, modifiers));
  }
/**
* Mouse double clicked
* @param {number} x
* @param {number} y
* @param {number} mouse_keys
* @param {number} modifiers
*/
  async onDoubleClick(x: number, y: number, mouse_keys: number, modifiers: number) {
	dispatch_response(await commands.on_double_click( x, y,  mouse_keys, modifiers));
  }
/**
* A keyboard button depressed within screenspace the bounds of the viewport
* @param {string} name
* @param {number} modifiers
*/
  async onKeyDown(name: string, modifiers: number) {
	dispatch_response(await commands.on_key_down( name, modifiers));
  }
/**
* A keyboard button released
* @param {string} name
* @param {number} modifiers
*/
  async onKeyUp(name: string, modifiers: number) {
	dispatch_response(await commands.on_key_up( name, modifiers));
  }
/**
* A text box was committed
* @param {string} new_text
*/
  async onChangeText(new_text: string) {
	  dispatch_response(await commands.on_change_text(new_text));
  }
/**
* A font has been downloaded
* @param {string} font_family
* @param {string} font_style
* @param {string} preview_url
* @param {Uint8Array} data
* @param {boolean} is_default
*/
  async onFontLoad(font_family: string, font_style: string, preview_url: string, data: Uint8Array, is_default: Promise<boolean>) {
	dispatch_response(await commands.on_font_load(font_family, font_style, preview_url, data, is_default));
  }
/**
* A text box was changed
* @param {string} new_text
*/
  async updateBounds(new_text: string) {
	  dispatch_response(await commands.update_bounds(new_text));
  }
/**
* Begin sampling a pixel color from the document by entering eyedropper sampling mode
*/
  async eyedropperSampleForColorPicker() {
	  dispatch_response(await invoke("eyedropper_sample_for_color_picker"));
  }
/**
* Update primary color with values on a scale from 0 to 1.
* @param {number} red
* @param {number} green
* @param {number} blue
* @param {number} alpha
*/
  async updatePrimaryColor(red: number, green: number, blue: number, alpha: number) {
	  dispatch_response(await commands.update_primary_color(red, green, blue, alpha));
  }
/**
* Update secondary color with values on a scale from 0 to 1.
* @param {number} red
* @param {number} green
* @param {number} blue
* @param {number} alpha
*/
  async updateSecondaryColor(red: number, green: number, blue: number, alpha: number) {
	  dispatch_response(await commands.update_secondary_color(red, green, blue, alpha));
  }
/**
* Paste layers from a serialized json representation
* @param {string} data
*/
  async pasteSerializedData(data: string) {
	  dispatch_response(await commands.paste_serialized_data(data));
  }
/**
* Modify the layer selection based on the layer which is clicked while holding down the <kbd>Ctrl</kbd> and/or <kbd>Shift</kbd> modifier keys used for range selection behavior
* @param {BigUint64Array} layer_path
* @param {boolean} ctrl
* @param {boolean} shift
*/
  async selectLayer(layer_path: BigUint64Array, ctrl: Promise<boolean>, shift: boolean) {
	  dispatch_response(await commands.select_layer(layer_path, ctrl, shift));

  }
/**
* Deselect all layers
*/
  async deselectAllLayers() {}
/**
* Move a layer to be next to the specified neighbor
* @param {BigUint64Array} folder_path
* @param {number} insert_index
*/
  async moveLayerInTree(folder_path: BigUint64Array, insert_index: number) {

	  dispatch_response(await commands.move_layer_in_tree(folder_path, insert_index));
  }
/**
* Set the name for the layer
* @param {BigUint64Array} layer_path
* @param {string} name
*/
  async setLayerName(layer_path: BigUint64Array, name: string) {

	  dispatch_response(await commands.set_layer_name(layer_path, name));
  }
/**
* Translates document (in viewport coords)
* @param {number} delta_x
* @param {number} delta_y
*/
  async translateCanvas(delta_x: number, delta_y: number) {

	  dispatch_response(await commands.translate_canvas(delta_x, delta_y));
  }
/**
* Translates document (in viewport coords)
* @param {number} delta_x
* @param {number} delta_y
*/
  async translateCanvasByFraction(delta_x: number, delta_y: number) {

	  dispatch_response(await commands.translate_canvas_by_fraction(delta_x, delta_y));
  }
/**
* Sends the blob URL generated by JS to the Image layer
* @param {bigint} document_id
* @param {BigUint64Array} layer_path
* @param {string} blob_url
* @param {number} width
* @param {number} height
*/
  async setImageBlobURL(document_id: bigint, layer_path: BigUint64Array, blob_url: string, width: number, height: number) {
	  dispatch_response(await commands.set_image_blob_url(document_id, layer_path, blob_url, width, height));
  }
/**
* Sends the blob URL generated by JS to the Imaginate layer in the respective document
* @param {bigint} document_id
* @param {BigUint64Array} layer_path
* @param {BigUint64Array} node_path
* @param {Uint8Array} image_data
* @param {number} width
* @param {number} height
*/
  async setImaginateImageData(document_id: bigint, layer_path: BigUint64Array, node_path: BigUint64Array, image_data: Uint8Array, width: number, height: number) {

	  dispatch_response(await commands.set_imaginate_image_data( document_id, layer_path, node_path, image_data, width, height ));
  }
/**
* Notifies the Imaginate layer of a new percentage of completion and whether or not it's currently generating
* @param {bigint} document_id
* @param {BigUint64Array} layer_path
* @param {BigUint64Array} node_path
* @param {number | undefined} percent
* @param {string} status
*/
  async setImaginateGeneratingStatus(document_id: bigint, layer_path: BigUint64Array, node_path: BigUint64Array, percent: number | undefined, status: string) {
	dispatch_response(await commands.set_imaginate_generating_status( document_id, layer_path, node_path,  percent,  status ));
  }
/**
* Notifies the editor that the Imaginate server is available or unavailable
* @param {boolean} available
*/
  async setImaginateServerStatus(available: Promise<boolean>) {

	  dispatch_response(await commands.set_imaginate_server_status( available ));
  }
/**
* Sends the blob URL generated by JS to the Imaginate layer in the respective document
* @param {bigint} document_id
* @param {BigUint64Array} layer_path
* @param {Uint8Array} image_data
* @param {number} width
* @param {number} height
* @param {BigUint64Array | undefined} imaginate_node
*/
  async processNodeGraphFrame(document_id: bigint, layer_path: BigUint64Array, image_data: Uint8Array, width: number, height: number, imaginate_node?: BigUint64Array) {

	  dispatch_response(await commands.process_node_graph_frame( document_id, layer_path, image_data, width, height,  imaginate_node ));
  }
/**
* Notifies the backend that the user connected a node's primary output to one of another node's inputs
* @param {bigint} output_node
* @param {number} output_node_connector_index
* @param {bigint} input_node
* @param {number} input_node_connector_index
*/
  async connectNodesByLink(output_node: bigint, output_node_connector_index: number, input_node: bigint, input_node_connector_index: number) {

	  dispatch_response(await commands.connect_nodes_by_link( output_node,  output_node_connector_index,  input_node,  input_node_connector_index));
  }
/**
* Shifts the node and its children to stop nodes going ontop of each other
* @param {bigint} node_id
*/
  async shiftNode(node_id: bigint) {

	  dispatch_response(await commands.shift_node(node_id));
  }
/**
* Notifies the backend that the user disconnected a node
* @param {bigint} node_id
* @param {number} input_index
*/
  async disconnectNodes(node_id: bigint, input_index: number) {

	  dispatch_response(await commands.disconnect_nodes(node_id, input_index));
  }
/**
* Check for intersections between the curve and a rectangle defined by opposite corners
* @param {Float64Array} bezier_x
* @param {Float64Array} bezier_y
* @param {number} top
* @param {number} left
* @param {number} bottom
* @param {number} right
* @returns {boolean}
*/
  async rectangleIntersects(bezier_x: Float64Array, bezier_y: Float64Array, top: number, left: number, bottom: number, right: number): Promise<boolean> {

	  return await commands.rectangle_intersects(bezier_x, bezier_y, top, left, bottom, right);
  }
/**
* Creates a new document node in the node graph
* @param {string} node_type
* @param {number} x
* @param {number} y
*/
  async createNode(node_type: string, x: number, y: number) {

	  dispatch_response(await commands.create_node(node_type, x, y));
  }
/**
* Notifies the backend that the user selected a node in the node graph
* @param {BigUint64Array} nodes
*/
  async selectNodes(nodes: BigUint64Array) {

	  dispatch_response(await commands.select_nodes(nodes));
  }
/**
* Pastes the nodes based on serialized data
* @param {string} serialized_nodes
*/
  async pasteSerializedNodes(serialized_nodes: string) {

	  dispatch_response(await commands.paste_serialized_nodes(serialized_nodes));
  }
/**
* Notifies the backend that the user double clicked a node
* @param {bigint} node
*/
  async doubleClickNode(node: bigint) {

	  dispatch_response(await commands.double_click_node(node));
  }
/**
* Notifies the backend that the selected nodes have been moved
* @param {number} displacement_x
* @param {number} displacement_y
*/
  async moveSelectedNodes(displacement_x: number, displacement_y: number) {

	  dispatch_response(await commands.move_selected_nodes(displacement_x, displacement_y));
  }
/**
* Toggle preview on node
* @param {bigint} node_id
*/
  async togglePreview(node_id: bigint) {
	  dispatch_response(await commands.toggle_preview(node_id));
  }
/**
* Pastes an image
* @param {Uint8Array} image_data
* @param {number} width
* @param {number} height
* @param {number | undefined} mouse_x
* @param {number | undefined} mouse_y
*/
  async pasteImage(image_data: Uint8Array, width: number, height: number, mouse_x?: number, mouse_y?: number) {

	  dispatch_response(await commands.paste_image(image_data, width, height, mouse_x, mouse_y));
  }
/**
* Toggle visibility of a layer from the layer list
* @param {BigUint64Array} layer_path
*/
  async toggleLayerVisibility(layer_path: BigUint64Array) {

	  dispatch_response(await commands.toggle_layer_visibility(layer_path));
  }
/**
* Toggle expansions state of a layer from the layer list
* @param {BigUint64Array} layer_path
*/
  async toggleLayerExpansion(layer_path: BigUint64Array) {

	  dispatch_response(await commands.toggle_layer_expansion(layer_path));
  }
}

const instance = new TauriHandle();

async function dispatch_response(response: any[]) {
	console.log(response);
	let array = response;
	for (let deserialized of array) {
		let messageType = Object.keys(deserialized)[0];
		if (typeof deserialized === "string") {
			messageType = deserialized;
		}
		console.log(deserialized);

		subscriptions.handleJsMessage(messageType, deserialized as unknown as Record<string, unknown>, {}, instance);
	}

}
