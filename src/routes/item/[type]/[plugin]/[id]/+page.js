export const prerender = false;
export function load({ params, url }) {
    const query = url.searchParams.get('item');
    let queryParsed = null;
    try {
        queryParsed = JSON.parse(query);
    } catch (e) {
        console.error(e);
        queryParsed = null;
    }
    
    return {
        id: params.id,
        plugin: params.plugin,
        type: params.type,
        item: queryParsed
    }
}