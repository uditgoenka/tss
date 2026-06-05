import { definePluginEntry } from "openclaw/plugin-sdk/plugin-entry";

function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\"'\"'`)}'`;
}

function wrap(command) {
  const trimmed = String(command).trim();
  if (!trimmed || trimmed.startsWith("tss ")) return command;
  return `tss run -- bash -lc ${shellQuote(command)}`;
}

export default definePluginEntry({
  id: "tss",
  name: "TSS",
  register(api) {
    api.on("before_tool_call", async (event) => {
      const command = event?.params?.command;
      if (typeof command !== "string") return {};

      return {
        params: {
          ...event.params,
          command: wrap(command),
        },
      };
    });
  },
});
