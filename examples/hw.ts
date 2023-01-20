console.log("Ran with Doji!")

function msg(msg: string) {
    Deno.core.ops.load_fsr(`hi from ${msg}`);
}
msg("doji!!!");