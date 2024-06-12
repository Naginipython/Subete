var fs = require('fs');

let plugin = {}

plugin.id = "MangaDex"
plugin.media_type = "manga";

plugin.search_url = "https://api.mangadex.org/manga?limit=100&includes[]=cover_art&includes[]=author&includes[]=artist&title={title}";
plugin.search = `
function search(html) {
    html = JSON.parse(html); 
    let data = [];
        for (let d of html['data']) {
            let temp = {};
            temp['id'] = d['id'];
            temp['title'] = d['attributes']['title']['en'];
            let filetemp = d['relationships'].filter(o => o.type == 'cover_art')[0];
            temp['img'] = \`https://uploads.mangadex.org/covers/\${temp['id']}/\${filetemp['attributes']['fileName']}\`;
            temp['plugin'] = 'MangaDex';
            temp['description'] = d['attributes']['description']['en'];
            temp['chapters'] = [];
            let author_names = d['relationships'].filter(x => x.type == 'author').map(y => y['attributes']['name']);
            let artist_names = d['relationships'].filter(x => x.type == 'artist').map(y => y['attributes']['name']);
            temp['authors'] = author_names.join(', ');
            temp['artists'] = artist_names.join(', ');
            data.push(temp);
        }
    return data;
}
`;
plugin.search_extra = {}

plugin.chapters_url = "https://api.mangadex.org/manga/{id}/feed?limit=500&order[chapter]=asc&translatedLanguage[]=en";
plugin.get_chapters = `
function getChapters(html) {
    html = JSON.parse(html); 
    return html['data'].map(e => {
      return {
          number: parseFloat(e['attributes']['chapter'])? parseFloat(e['attributes']['chapter']) : 0.0,
          id: e['id'],
          title: e['attributes']['title'] == '' || e['attributes']['title'] == null? \`Chapter \${e['attributes']['chapter']}\` : e['attributes']['title'],
          page: 1,
          completed: false
      }
    });
  }
`;
plugin.chapters_extra = {}

plugin.pages_url = "https://api.mangadex.org/at-home/server/{id}";
plugin.get_pages = `
function getChapterPages(html) {
    html = JSON.parse(html); 
    let hash = html['chapter']['hash'];
    let data = html['chapter']['data'];
    return data.map(x => \`https://uploads.mangadex.org/data/\${hash}/\${x}\`);
  }
`;
plugin.pages_extra = {}

// Removes /n, extra spaces, and '\' needed for js things
for (const [key, val] of Object.entries(plugin)) {
  if (typeof val == "string") {
    plugin[key] = val.replaceAll('\n', '').replace(/\s+/g, ' ');
  }
}

async function tests() {
  // Testing if search works
  const search_res = await fetch(plugin.search_url.replace("{title}", "one"));
  const search_data = await search_res.text();
  const search_test = eval(plugin.search + `search(${JSON.stringify(search_data)})`);
  if (
    !search_test[0].hasOwnProperty("id") ||
    !search_test[0].hasOwnProperty("title") ||
    !search_test[0].hasOwnProperty("img") ||
    !search_test[0].hasOwnProperty("plugin") ||
    !search_test[0].hasOwnProperty("authors") ||
    !search_test[0].hasOwnProperty("artists") ||
    !search_test[0].hasOwnProperty("description") ||
    !search_test[0].hasOwnProperty("chapters")
  ) {
    console.log("Search test failed; Missing field.");
    return false;
  }
  
  // Testing if getChapters works
  const chap_res = await fetch(plugin.chapters_url.replace("{id}", search_test[0].id));
  const chap_data = await chap_res.text();
  const chap_test = eval(plugin.get_chapters + `getChapters(${JSON.stringify(chap_data)})`);
  if (
    !chap_test[0].hasOwnProperty("id") ||
    !chap_test[0].hasOwnProperty("number") ||
    !chap_test[0].hasOwnProperty("title") ||
    !chap_test[0].hasOwnProperty("page") ||
    !chap_test[0].hasOwnProperty("completed")
  ) {
    console.log("Search test failed; Missing field.");
    return false;
  }

  // Testing if getChapters works
  const page_res = await fetch(plugin.pages_url.replace("{id}", chap_test[0].id));
  const page_data = await page_res.text();
  const page_test = eval(plugin.get_pages + `getChapterPages(${JSON.stringify(page_data)})`);
  if (page_test.length <= 0) {
    console.log("Search test failed; Missing field.");
    return false;
  }
  return true;
}

// Tests plugin, then sends to file
tests().then((result) => {
  // Writes to a file, to be inserted as a plugin
  if (result) {
    fs.writeFile(`plugins/${plugin.id}.json`, JSON.stringify(plugin), (error) => {
        if (error) {
          console.error(error);
          throw error;
        }
    });
  }
})