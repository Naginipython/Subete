/**
 * Takes in a query, and returns an array of objects with multiple extentions name and data.
 * 
 * @param {string} query - String to be used to query the source
 * @returns {Array<{
 *      extention: string,
 *      data: Array<{
 *          id: string,
 *          title: string,
 *          img: string,
 *          extention: string,
 *          authors: string,
 *          artists: string,
 *          description: string,
 *          chapters: Array<{
 *              id: string,
 *              num: number,
 *              title: string,
 *              page: number
 *          }>
 *      }>
 * }>} - An array of objects, notably the top level contains the extention & data
 */
export async function searchManga(query) {
    const modules = import.meta.glob('./*.js');

    let results = [];
    const modulePromises = Object.entries(modules).map(async ([path, moduleImporter]) => {
        const { search } = await moduleImporter();
        return search(query);
    });

    results = await Promise.all(modulePromises);
    return results.flat(1);
}

/**
 * Takes in a source string and an id that the source can use to retrieve the chapter details for a series.
 * 
 * @param {string} source - String representing a source. Will be combined with './' and '.js', so keep the extention name given in search
 * @param {string} id - String of the id of a given manga. Used for finding the website details
 * @returns {Array<{
 *      id: string,
 *      num: number,
 *      title: string,
 *      page: number
 * }>} - Array chapter details
 */
export async function getChapters(source, id) {
    const modules = import.meta.glob('./*.js');

    for (const [key, value] of Object.entries(modules)) {
        if (key == `./${source.toLowerCase()}.js`) {
            const { getChapters } = await import(/* @vite-ignore */key);
            return await getChapters(id);
        }
    }
    return [];
}

/**
 * Takes in a source string and an id that the source can use to retrieve the chapter pages for a series.
 * 
 * @param {string} source - String representing a source. Will be combined with './' and '.js', so keep the extention name given in search
 * @param {string} id - String of the id of a given manga. Used for finding the website details
 * @returns {Array<string>} - Array of strings for each page link
*/
export async function getChapterPages(source, id) {
    const modules = import.meta.glob('./*.js');

    for (const [key, value] of Object.entries(modules)) {
        if (key == `./${source.toLowerCase()}.js`) {
            const { getChapterPages } = await import(/* @vite-ignore */key);
            return await getChapterPages(id);
        }
    }
    return [];
}