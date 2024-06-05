var fs = require('fs');

// This helps one create a JSON that can be inputted for a plugin.
let result = {}

// Name for your plugin
result.id = ""; 
// result.id = "MangaDex"

// Url for your plugin. Note that '{title}' is needed to query
result.search_url = "{title}"; 
// result.search_url = "https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}";

// JS code needed to search. Will need a param of the raw GET of the search_url, and return:
// LibraryItem:
// {
//     id: String,
//     title: String,
//     img: String,
//     extension: String,
//     authors: String,
//     artists: String,
//     description: String OR none,
//     chapters: [ ChapterItem ]
// }
result.search = "";
// result.search = `
// function search(json) {
//     let data = [];
//         for (let d of json['data']) {
//             let temp = {};
//             temp['id'] = d['id'];
//             temp['title'] = d['attributes']['title']['en'];
//             let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0];
//             temp['img'] = \`https://uploads.mangadex.org/covers/\${temp['id']}/\${filetemp['attributes']['fileName']}\`;
//             temp['extension'] = 'MangaDex';
//             temp['description'] = d['attributes']['description']['en'];
//             temp['chapters'] = [];
//             let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']);
//             let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']);
//             temp['authors'] = author_names.join(', ');
//             temp['artists'] = artist_names.join(', ');
//             data.push(temp);
//         }
//     return data;
// }
// `

// Get Chapters url. Note that '{id}' is needed to query
result.chapters_url = "{id}";
// result.chapters_url = "https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en";

// JS code needed to get chapter data. Will need a param of the raw GET of the chapters_url, and return:
// ChapterItem:
// [
//     {
//         id: String,
//         number: Float,
//         title: String,
//         page: Number,
//         completed: Boolean
//     }
// ]
result.get_chapters = "";
// result.get_chapters = `
// function getChapters(json) {
//     return json['data'].map(e => {
//       return {
//           number: parseInt(e['attributes']['chapter']),
//           id: e['id'],
//           title: e['attributes']['title'] == '' || e['attributes']['title'] == null? `Chapter ${e['attributes']['chapter']}` : e['attributes']['title'],
//           page: 1,
//           completed: false
//       }
//     });
//   }
// `;

// Get Pages url. Note that '{id}' is needed to query
result.pages_url = "{id}";
// result.pages_url = "https://api.mangadex.org/at-home/server/{id}";

// JS code needed to get page links. Will need a param of the raw GET of the chapters_url, and return:
// [ String ]
result.get_pages = "";
// result.get_pages = `
// function getChapterPages(json) {
//     let hash = json['chapter']['hash'];
//     let data = json['chapter']['data'];
//     return data.map(x => \`https://uploads.mangadex.org/data/\${hash}/\${x}\`);
//   }
// `;

fs.writeFile(`${result.id}.json`, JSON.stringify(result), (error) => {
    // throwing the error
    // in case of a writing problem
    if (error) {
      // logging the error
      console.error(error);
  
      throw error;
    }
});