/**
 * Takes in a query, and returns an object with the extention name and data.
 * 
 * @param {string} query - String to be used to query the source
 * 
 * @returns {Array<{
 *      id: string,
 *      title: string,
 *      img: string,
 *      extention: string,
 *      authors: string,
 *      artists: string,
 *      description: string,
 *      chapters: Array<{
 *          id: string,
 *          num: number,
 *          title: string,
 *          page: number
 *      }>
 * }>} 
 */
export async function search(query) {
    let data = [];
    let body = await fetch(`https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title=${query}`);
    let res = await body.json();

    for (let d of res['data']) {
        let temp = {};
        temp['id'] = d['id'];
        temp['title'] = d['attributes']['title']['en'];
        let filetemp = d["relationships"].filter(o => o.type == "cover_art")[0];
        temp['img'] = `https://uploads.mangadex.org/covers/${temp['id']}/${filetemp['attributes']['fileName']}`;
        temp['extention'] = "MangaDex";
        temp['description'] = d['attributes']['description']['en'];
        temp['chapters'] = [];
        let author_names = d['relationships'].filter(x => x.type == "author").map(y => y['attributes']['name']);
        let artist_names = d['relationships'].filter(x => x.type == "artist").map(y => y['attributes']['name']);
        temp['authors'] = author_names.join(', ');
        temp['artists'] = artist_names.join(', ');
        data.push(temp);
    }
    return data;
}

/**
 * Takes in an id that the source can use to retrieve the chapter details for a series.
 * 
 * @param {string} id - String of the id of a given manga. Used for finding the website details
 * @returns {Array<{
 *      id: string,
 *      number: number,
 *      title: string,
 *      page: number
 * }>} - Array chapter details
 */
export async function getChapters(id) {
    let body = await fetch(`https://api.mangadex.org/manga/${id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en`);
    let res = await body.json();
    return res['data'].map(e => {
        return {
            number: parseInt(e['attributes']['chapter']),
            id: e['id'],
            title: e['attributes']['title'] == "" || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'],
            page: 1,
        }
    });
}

/**
 * Takes in an id that the source can use to retrieve the chapter pages for a series.
 * 
 * @param {string} id - String to be used to query the source
 * 
 * @returns {Array<string>} 
 */
export async function getChapterPages(id) {
    let body = await fetch(`https://api.mangadex.org/at-home/server/${id}`);
    let res = await body.json();
    let hash = res['chapter']['hash'];
    let data = res['chapter']['data'];
    return data.map(x => `https://uploads.mangadex.org/data/${hash}/${x}`);
}