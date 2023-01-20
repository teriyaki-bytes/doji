console.log("Ran with Doji!")

function msg() {
    Deno.core.ops.load_fsr("hi from TS");
}
msg();