let history = [];
let temp_more_exceptions = ["/settings", "/history"];

const pushState = (nav) => {
    if (history[history.length-1] != nav) {
        if (history.length>0) {
            let nav_deep = nav.split('/').length;
            let pages_deep = history[history.length-1].split('/').length;
            if (pages_deep == nav_deep && !temp_more_exceptions.some(l => nav==l)) {
                history.pop();
            }
            history.push(nav);
        } else {
            history.push(nav);
        }
    }
} 
const back = () => {
    history.pop();
    return history[history.length-1];
}
const reset = () => {
    history = [];
}
const peek = () => {
    return history[history.length-1];
}
const print = () => {
    console.log(history);
}

export default {
    pushState,
    back,
    reset,
    peek,
    print,
};