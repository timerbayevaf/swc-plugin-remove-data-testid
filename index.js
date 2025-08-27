const path = require("path");

module.exports = function swcPluginRemoveDataTestId() {
  return [path.join(__dirname, "wasm/swc_plugin_remove_data_testid.wasm"), {}];
};
