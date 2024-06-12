var fs = require('fs');
let plugin = {}

plugin.id = "Asura Light Novel";

plugin.search_url = "https://asuralightnovel.com/?s={title}&post_type=wp-manga";

plugin.search = `
function search(html) {
  let data = [];
  html = html.replaceAll('\\n', ' ').replace(new RegExp('\\\\s+', 'g'), ' ');
  const regex = new RegExp(\`<div class="row c-tabs-item__content">(.*?)</span></div></div></div></div></div>\`, 'gi');
  let data_to_parse = [];
  while ((match = regex.exec(html)) !== null) {
    data_to_parse.push(match[1]);
  }
  for (item of data_to_parse) {
    let lib_item = {};
    const id_regex = new RegExp(\`<h3 class="h4"><a href="https://asuralightnovel.com/novel/(.*?)/">(.*?)</a>\`, 'gi');
    let reg = id_regex.exec(item);
    lib_item.id = reg[1];
    lib_item.title = reg[2];
    const img_regex = new RegExp(\`data-src="(.*?)"\`, 'gi');
    lib_item.img = img_regex.exec(item)[1];
    const author_regex = new RegExp(\`<a href="https://asuralightnovel.com/novel-author/.*?/">(.*?)</a>\`, 'gi');
    lib_item.authors = author_regex.exec(item)[1];
    const artist_regex = new RegExp(\`<a href="https://asuralightnovel.com/novel-artist/.*?/">(.*?)</a>\`, 'gi');
    lib_item.artists = artist_regex.exec(item)[1];
    lib_item.description = '';
    lib_item.plugin = "Asura Light Novel";
    lib_item.chapters = [];
    data.push(lib_item);
  }
  return data;
}
`;
plugin.search_extra = {};

plugin.chapters_url = "https://asuralightnovel.com/novel/{id}/ajax/chapters/";
plugin.get_chapters = `
function getChapters(html) {
  let data = [];
  html = html.replaceAll('\\n', ' ').replace(new RegExp('\\\\s+', 'g'), ' ');
  const regex = new RegExp(\`<li class="wp-manga-chapter "> (.*?) </li>\`, 'gi');
  let data_to_parse = [];
  while ((match = regex.exec(html)) !== null) {
    data_to_parse.push(match[1]);
  }

  for (item of data_to_parse) {
    let lib_item = {};
    const id_regex = new RegExp(\`<a href="https://asuralightnovel.com/novel/(.*?)/(.*?)/"> (.*?) </a>\`, 'gi');
    let reg = id_regex.exec(item);
    lib_item.id = reg[1] + "/" + reg[2];
    lib_item.title = reg[3];
    lib_item.number = parseInt(lib_item.title.match(new RegExp(\`\\\\d+\`))[0]);
    lib_item.page = 1;
    lib_item.completed = false;
    data.push(lib_item);
  }
  return data;
}`;
plugin.chapters_extra = {
  request: 'post'
}

plugin.pages_url = "https://asuralightnovel.com/novel/{id}";
plugin.get_pages = `
function getChapterPages(html) {
  let data = [];
  html = html.replaceAll('\\n', ' ').replace(new RegExp('\\\\s+', 'g'), ' ');
  const regex = new RegExp(\`<p>(.*?)</p>\`, 'gi');
  while ((match = regex.exec(html)) !== null) {
    if (!match[1].includes(\`<\`) && match[1] != '' && match[1] != "© 2021 Asura Light Novel Inc. All rights reserved") {
      data.push(match[1]);
    }
  }
  return data;
}
`;
plugin.pages_extra = {};

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
  // console.log(search_test);
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
  let chap_res;
  if (plugin.chapters_extra.hasOwnProperty("request")) {
    chap_res = await fetch(plugin.chapters_url.replace("{id}", search_test[0].id), {
      method: 'POST',
      headers: {
        'Content-Type': 'text/html'
      }
    });
  } else {
    chap_res = await fetch(plugin.chapters_url.replace("{id}", search_test[0].id));
  }
  const chap_data = await chap_res.text();
  const chap_test = eval(plugin.get_chapters + `getChapters(${JSON.stringify(chap_data)})`);
  if (
    !chap_test[0].hasOwnProperty("id") ||
    !chap_test[0].hasOwnProperty("number") ||
    !chap_test[0].hasOwnProperty("title") ||
    !chap_test[0].hasOwnProperty("page") ||
    !chap_test[0].hasOwnProperty("completed")
  ) {
    console.log("Chap test failed; Missing field.");
    return false;
  }

  // Testing if getChapters works
  const page_res = await fetch(plugin.pages_url.replace("{id}", chap_test[0].id));
  const page_data = await page_res.text();
  const page_test = eval(plugin.get_pages + `getChapterPages(${JSON.stringify(page_data)})`);
  if (page_test.length <= 0) {
    console.log("Page test failed; Missing field.");
    return false;
  }
  return true;
}

// Tests plugin, then sends to file
tests().then((result) => {
  // Writes to a file, to be inserted as a plugin
  if (result) {
    fs.writeFile(`plugins/${plugin.id.replaceAll(' ', '_')}.json`, JSON.stringify(plugin), (error) => {
        if (error) {
          console.error(error);
          throw error;
        }
    });
  }
})