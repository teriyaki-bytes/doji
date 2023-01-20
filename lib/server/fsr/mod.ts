
export function serve(dir: string) {
    console.log("This worked!")
    Deno.core.ops.load_fsr(dir)
}