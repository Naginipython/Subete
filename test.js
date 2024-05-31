function groupByType(array) {
    return array.reduce((result, item) => {
        const extention = item.extention;
        if (!result[extention]) {
            result[extention] = {extention: extention, data: []};
        }
        result[extention]['data'].push(item);
        return result;
    }, {});
}

// To convert the result object into an array of arrays
function groupByTypeAsArrays(array) {
    const grouped = groupByType(array);
    console.log(grouped);
    return Object.values(grouped);
}

// Example usage:
const data = [
    { extention: 'A', value: 1 },
    { extention: 'B', value: 2 },
    { extention: 'A', value: 3 },
    { extention: 'C', value: 4 },
    { extention: 'B', value: 5 }
];

const groupedData = groupByTypeAsArrays(data);
console.log("test");
console.log(groupedData);