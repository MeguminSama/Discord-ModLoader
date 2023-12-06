/// BEGIN MODHOOK INFO ///
const CUSTOM_DATA_DIR = process.env.MODHOOK_CUSTOM_DATA_DIR;
const MOD_ENTRYPOINT = process.env.MODHOOK_MOD_ENTRYPOINT;
const IS_MOONLIGHT = process.env.MODHOOK_IS_MOONLIGHT;
/// END MODHOOK INFO ///

if (CUSTOM_DATA_DIR) {
	const { app } = require("electron");
	const customAppDir =
		app.getPath("appData") + "\\DiscordModHook\\AppData\\" + CUSTOM_DATA_DIR;
	const _setPath = app.setPath;

	app.setPath = function (name, path) {
		if (name === "userData") {
			_setPath.call(app, name, customAppDir);
		} else {
			_setPath.call(app, name, path);
		}
	};

	app.setPath("userData", customAppDir);
}

if (IS_MOONLIGHT) {
	require(MOD_ENTRYPOINT).inject(
		require("path").resolve(__dirname, "..\\_app.asar")
	);
} else {
	require(MOD_ENTRYPOINT);
}
