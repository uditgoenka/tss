export const TssPlugin = async () => {
  return {
    "tool.execute.before": async (input, output) => {
      if (!input || input.tool !== "bash") return
      if (!output || !output.args || typeof output.args.command !== "string") return

      const command = output.args.command.trim()
      if (!command || command.startsWith("tss ")) return

      output.args.command = `tss run -- bash -lc ${shellQuote(output.args.command)}`
    },
  }
}

function shellQuote(value) {
  return `'${String(value).replace(/'/g, `'\"'\"'`)}'`
}
