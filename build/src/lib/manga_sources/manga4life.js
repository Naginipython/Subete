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
*      chapters: Array<{
*          id: string,
*          num: number,
*          title: string,
*          page: number
*      }>
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
            console.log(retrieved.length)
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
*      id: string,
*      number: number,
*      title: string,
*      page: number
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
         "id": `${id}-chapter-${num}`,
         "number": num+decimal,
         "title": e['ChapterName'] == null? "" : e['ChapterName'],
         "page": 1
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
      const pattern = /vm\.CHAPTERS\s*=\s*(\[.*?\]);/s;
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
   // todo: use regex to get the num at the end of 'id'. remove period, or multiply by 10. make it 5 digits (pad 0s) with a 1 in front
   // let curr_chap_obj = 

   // todo: with 'curr_chap_obj', get 'total_chap_num'. Create an array of that 'num' links such as this: `https://manga4life.com/read-online/${id}-page-${num}.html`

   let data = [];
   
   return data;
}