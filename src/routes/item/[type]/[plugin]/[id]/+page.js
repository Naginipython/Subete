export const prerender = false;
export function load({ params }) {
    return {
        id: params.id,
        plugin: params.plugin,
        type: params.type
    }
}
