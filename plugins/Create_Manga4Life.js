var fs = require('fs');

// This helps one create a JSON that can be inputted for a plugin.
let plugin = {};

plugin.id = "Manga4Life"; 

plugin.media_type = "manga"

plugin.search_url = "https://manga4life.com/search/?sort=s&desc=false&name={title}"; 

plugin.search = `
function search(json) {
  let data = [];
  let retrieved;
  const keyword = 'vm.Directory =';
  const endChar = ']';
  const startIndex = json.indexOf(keyword);
  if (startIndex !== -1) {
      const remainingString = json.substring(startIndex + keyword.length).trim();
      const endIndex = remainingString.indexOf(endChar+';');
      if (endIndex !== -1) {
        retrieved = JSON.parse(remainingString.substring(0, endIndex + 1).trim());
      }
  }
  let query;
  const keyword2 = 'vm.Search =';
  const endChar2 = '}';
  const startIndex2 = json.indexOf(keyword2);
  if (startIndex2 !== -1) {
      const remainingString2 = json.substring(startIndex2 + keyword2.length).trim();
      const endIndex2 = remainingString2.indexOf(endChar2+';');
      if (endIndex2 !== -1) {
        query = JSON.parse(remainingString2.substring(0, endIndex2 + 1).trim()).SeriesName;
      }
  }
  let filtered_retrieve = retrieved.filter(e => 
    e['s'].toLowerCase().includes(query) || 
    e['al'].some(f => f.toLowerCase().includes(query)) ||
    e['a'].some(f => f.toLowerCase().includes(query))
  );
    
  return filtered_retrieve.map(e => {
    return {
       'id': e['i'],
       'title': e['s'],
       'img': \`https://temp.compsci88.com/cover/\${e['i']}.jpg\`,
       'plugin': 'Manga4Life',
       'authors': e['a'].join(', '),
       'artists': '',
       'description': '',
       'chapters': []
    };
 });
}
`;
plugin.search_extra = {};

plugin.chapters_url = "https://manga4life.com/manga/{id}";

plugin.get_chapters = `
function getChapters(json) {
  let retrieved;
  const keyword = 'vm.Chapters =';
  const endChar = ']';
  const startIndex = json.indexOf(keyword);
  if (startIndex !== -1) {
      const remainingString = json.substring(startIndex + keyword.length).trim();
      const endIndex = remainingString.indexOf(endChar+';');
      if (endIndex !== -1) {
        retrieved = JSON.parse(remainingString.substring(0, endIndex + 1).trim());
      }
  }
  let id;
  const keyword2 = 'vm.IndexName =';
  const endChar2 = '\"';
  const startIndex2 = json.indexOf(keyword2);
  if (startIndex2 !== -1) {
      const remainingString2 = json.substring(startIndex2 + keyword2.length).trim();
      const endIndex2 = remainingString2.indexOf(endChar2+';');
      if (endIndex2 !== -1) {
        id = JSON.parse(remainingString2.substring(0, endIndex2 + 1).trim());
      }
  }
  let data = retrieved.map(e => {
    let decimal = parseFloat('0.'+e['Chapter'][5]);
    let num = parseInt(e['Chapter'].slice(1, 5));
    return {
       'id': \`\${id}-chapter-\${num+decimal}\`,
       'number': num+decimal,
       'title': e['ChapterName'] == null? '' : e['ChapterName'],
       'page': 1,
       'completed': false
    }
  });
 
  return data;
}
`;
plugin.chapters_extra = {};

plugin.pages_url = "https://manga4life.com/read-online/{id}-page-1.html";

plugin.get_pages = `
function getChapterPages(json) {
  let retrieved;
  const keyword = 'vm.CurChapter =';
  const endChar = '}';
  const startIndex = json.indexOf(keyword);
  if (startIndex !== -1) {
      const remainingString = json.substring(startIndex + keyword.length).trim();
      const endIndex = remainingString.indexOf(endChar+';');
      if (endIndex !== -1) {
        retrieved = JSON.parse(remainingString.substring(0, endIndex + 1).trim());
      }
  }
  let link;
  const keyword2 = 'vm.CurPathName =';
  const endChar2 = '\"';
  const startIndex2 = json.indexOf(keyword2);
  if (startIndex2 !== -1) {
      const remainingString2 = json.substring(startIndex2 + keyword2.length).trim();
      const endIndex2 = remainingString2.indexOf(endChar2+';');
      if (endIndex2 !== -1) {
        link = JSON.parse(remainingString2.substring(0, endIndex2 + 1).trim());
      }
  }
  let id;
  const keyword3 = 'vm.IndexName =';
  const endChar3 = '\"';
  const startIndex3 = json.indexOf(keyword3);
  if (startIndex3 !== -1) {
      const remainingString3 = json.substring(startIndex3 + keyword3.length).trim();
      const endIndex3 = remainingString3.indexOf(endChar3+';');
      if (endIndex3 !== -1) {
        id = JSON.parse(remainingString3.substring(0, endIndex3 + 1).trim());
      }
  }
  let chapter = retrieved.Chapter.slice(1,-1);
  let period = retrieved.Chapter[retrieved.Chapter.length -1];
  let pages = parseInt(retrieved.Page);
  let data = [];
  for (let i=1; i < pages+1; i++) {
    if (period != 0) {
        let newChap = chapter + '.' + period;
        let pad_math = 5 + newChap.split('.')[1].length; 
        data.push(\`https://\${link}/manga/\${id}/\${newChap.padStart(pad_math, '0')}-\${i.toString().padStart(3, '0')}.png\`)
    } else {
        data.push(\`https://\${link}/manga/\${id}/\${chapter.padStart(4, '0')}-\${i.toString().padStart(3, '0')}.png\`)
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
  const search_res = await fetch(plugin.search_url.replace("{title}", "mashle"));
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