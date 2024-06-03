import { http } from '@tauri-apps/api';
import { ResponseType } from '@tauri-apps/api/http';

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
*      chapters: Array<{}>
* }>} 
*/
export async function search(query) {
   let retrieved;
   try {
      const response = await http.fetch(`https://manga4life.com/search/?sort=s&desc=false&name=${query}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'text/html'
        },
        responseType: ResponseType.Text
      });
      const rawHtml = response.data;
      
      // Within the large raw HTML, there is a variable called 'vm.Directory'. That variables contains all the data.
      const pattern = /vm\.Directory\s*=\s*(\[.*?\]);/s;
      const match = rawHtml.match(pattern);
      if (match && match[1]) {
         const jsCode = match[1];
         try {
            retrieved = JSON.parse(jsCode);
          } catch (error) {
            console.error('Error parsing JSON:', error);
          }
      }
   } catch (error) {
      console.error('Error fetching data:', error);
   }
   // Manga4Life works via containing entire database, then filtering.
   let filtered_retrieve = retrieved.filter(e => 
      e['s'].toLowerCase().includes(query) || 
      e['al'].some(f => f.toLowerCase().includes(query)) ||
      e['a'].some(f => f.toLowerCase().includes(query))
   );

   //  [ { id,title,img,extention,authors,artists,description,chapters }]
   let data = filtered_retrieve.map(async e => {
      let desc = "";
      try {
         const response = await http.fetch(`https://manga4life.com/manga/${e['i']}`, {
           method: 'GET',
           headers: {
             'Content-Type': 'text/html'
           },
           responseType: ResponseType.Text
         });
         const rawHtml = response.data;
         const parser = new DOMParser();
         const doc = parser.parseFromString(rawHtml, 'text/html');
         desc = doc.querySelector('div.top-5.Content').innerText;
      } catch {
         console.error('Error fetching description:', error);
      }
      return {
         "id": e['i'],
         "title": e['s'],
         "img": `https://temp.compsci88.com/cover/${e['i']}.jpg`,
         "extention": "Manga4Life",
         "authors": e['a'].join(", "),
         "artists": "",
         "description": desc,
         "chapters": []
      }
   });
   return await Promise.all(data);
}

/**
* Takes in an id that the source can use to retrieve the chapter details for a series.
* 
* @param {string} id - String of the id of a given manga. Used for finding the website details
* @returns {Array<{
*     id: string,
*     number: number,
*     title: string,
*     page: number,
*     completed: boolean
* }>} - Array chapter details
*/
export async function getChapters(id) {
   let retrieved;
   try {
      const response = await http.fetch(`https://manga4life.com/manga/${id}`, {
        method: 'GET',
        headers: {
          'Content-Type': 'text/html'
        },
        responseType: ResponseType.Text
      });
      const rawHtml = response.data;
      
      // Within the large raw HTML, there is a variable called 'vm.Chapters'. That variables contains chapter data.
      const pattern = /vm\.Chapters\s*=\s*(\[.*?\]);/s;
      const match = rawHtml.match(pattern);
      if (match && match[1]) {
         const jsCode = match[1];
         try {
            retrieved = JSON.parse(jsCode);
          } catch (error) {
            console.error('Error parsing JSON:', error);
          }
      }
   } catch (error) {
      console.error('Error fetching data:', error);
   }
   // [{id,num,title,page}]
   let data = retrieved.map(e => {
      let decimal = parseFloat("0."+e['Chapter'][5]);
      let num = parseInt(e['Chapter'].slice(1, 5));
      return {
         "id": `${id}-chapter-${num+decimal}`,
         "number": num+decimal,
         "title": e['ChapterName'] == null? "" : e['ChapterName'],
         "page": 1,
         "completed": false
      }
   });
   
   return data;
}

/**
* Takes in an id that the source can use to retrieve the chapter pages for a series.
* 
* @param {string} id - String to be used to query the source
* 
* @returns {Array<string>} 
*/
export async function getChapterPages(id) {
   let retrieved;
   let link;
   try {
      const response = await http.fetch(`https://manga4life.com/read-online/${id}-page-1.html`, {
        method: 'GET',
        headers: {
          'Content-Type': 'text/html'
        },
        responseType: ResponseType.Text
      });
      const rawHtml = response.data;
      
      // Within the large raw HTML, there is a variable called 'vm.CHAPTERS'. That variables contains chapter data, or more relevant, how many pages per chapter
      const pattern = /vm\.CurChapter\s*=\s*(\{.*?\});/s;
      const match = rawHtml.match(pattern);
      if (match && match[1]) {
         const jsCode = match[1];
         try {
            retrieved = JSON.parse(jsCode);
          } catch (error) {
            console.error('Error parsing JSON:', error);
          }
      }

      // Gets the link data
      // CurPathName
      const pattern2 = /vm\.CurPathName\s*=\s*(\".*?\");/s;
      const match2 = rawHtml.match(pattern2);
      if (match2 && match2[1]) {
         const jsCode = match2[1];
         try {
            console.log(jsCode)
            link = JSON.parse(jsCode);
          } catch (error) {
            console.error('Error parsing JSON:', error);
          }
      }
   } catch (error) {
      console.error('Error fetching data:', error);
   }

   

   // prep
   let split_id = id.split("-chapter-");
   let pages = parseInt(retrieved.Page);
   let data = [];

   // creates the links
   for (let i=1; i < pages+1; i++) {
      if (split_id[1].includes(".")) {
         // 5 is the 4 number length the site uses a lot, plus the period. Just in case a chapter is multiple decimal long
         let pad_math = 5 + split_id[1].split(".")[1].length; 
         data.push(`https://${link}/manga/${split_id[0]}/${split_id[1].padStart(pad_math, '0')}-${i.toString().padStart(3, '0')}.png`)
      } else {
         data.push(`https://${link}/manga/${split_id[0]}/${split_id[1].padStart(4, '0')}-${i.toString().padStart(3, '0')}.png`)
      }
   }
   
   return data;
}