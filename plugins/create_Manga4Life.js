var fs = require('fs');

// This helps one create a JSON that can be inputted for a plugin.
let result = {}

// Name for your plugin
result.id = "Manga4Life"; 

result.search_url = "https://manga4life.com/search/?sort=s&desc=false&name={title}"; 

result.search = `
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
       'extension': 'Manga4Life',
       'authors': e['a'].join(', '),
       'artists': '',
       'description': '',
       'chapters': []
    }
 });
}
`;

result.chapters_url = "https://manga4life.com/manga/{id}";

result.get_chapters = `
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


result.pages_url = "https://manga4life.com/read-online/{id}-page-1.html";


result.get_pages = `
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
  let pages = parseInt(retrieved.Page);
  let data = [];
  for (let i=1; i < pages+1; i++) {
    if (chapter.includes('.')) {
        let period = retrieved.Chapter[retrieved.Chapter.length -1];
        chapter = chapter + '.' + period;
        let pad_math = 5 + chapter.split('.')[1].length; 
        data.push(\`https://\${link}/manga/\${id}/\${chapter.padStart(pad_math, '0')}-\${i.toString().padStart(3, '0')}.png\`)
    } else {
        data.push(\`https://\${link}/manga/\${id}/\${chapter.padStart(4, '0')}-\${i.toString().padStart(3, '0')}.png\`)
    }
  }
  return data;
}
`;



// Removes /n, extra spaces, and '\' needed for js things
for (const [key, val] of Object.entries(result)) {
  result[key] = val.replaceAll('\n', '').replace(/\s+/g, ' ');
}
// Writes to a file, to be inserted as a plugin
fs.writeFile(`${result.id}.json`, JSON.stringify(result), (error) => {
    if (error) {
      console.error(error);
      throw error;
    }
});