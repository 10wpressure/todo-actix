if (localStorage.getItem('user-token') == null) {
    window.location.replace(`${document.location.origin}/login/`);
} else {
    let cachedData = Date.parse(localStorage.getItem('item-cache-date'));
    let now = new Date();
    let difference = Math.round((now - cachedData)/(1000));

    if (difference <= 120) {
        runRenderProcess(JSON.parse(localStorage.getItem('item-cache-data')));
    } else {
        getItems();
    }
}

/**
 * Fires a series of render processes to fill out the main view.
 *
 * @param data {JSON} data from the API on the to-do items
 * @returns null
 */
function runRenderProcess(data) {
    renderItems(data["pending_items"], "edit", "pendingItems", editItem);
    renderItems(data["done_items"], "delete", "doneItems", deleteItem);
    document.getElementById("completeNum").innerHTML = data["done_item_count"];
    document.getElementById("pendingNum").innerHTML = data["pending_item_count"];
}


/**
 * Renders the to do items from the backend into a HTML div.
 *
 * @param items {Array} - list of to do items
 * @param processType {String} - the type of process that the button belonging to the to do item
 * @param elementId {String} - the id of the HTML element that the items will be inserted
 * @param processFunction {Function: editItem | deleteItem} - function that is fired once the button is clicked
 */
function renderItems(items, processType, elementId, processFunction) {
    let placeholder = "<div>";
    const itemsMeta = [];

    items.forEach((item) => {
        let title = item.title;
        let placeholderId = `${processType}-${title.replaceAll(" ", "-")}`;
        placeholder += `<div class="itemContainer"><p>${title}</p><div class="actionButton" id="${placeholderId}"> ${processType} </div></div>`;
        itemsMeta.push({ id: placeholderId, title });
    });
    placeholder += "</div>";
    document.getElementById(elementId).innerHTML = placeholder;

    itemsMeta.forEach((item) => {
        document.getElementById(item.id).addEventListener("click", processFunction);
    });
}

/**
 * Packages an API call ready to be sent.
 *
 * @param url {String} - the URL endpoint for the API call
 * @param method {String} - the method of the API call => POST, GET, PUT
 * @returns {XMLHttpRequest} - the API packaged API request
 */
function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;
    xhr.addEventListener('readystatechange', function() {
        if (this.readyState === this.DONE) {
            if (this.status === 401) {
                window.location.replace(`${document.location.origin}/login/`);
            } else {
                runRenderProcess(JSON.parse(this.responseText));
                localStorage.setItem("item-cache-date", new Date());
                localStorage.setItem("item-cache-data", this.responseText);
            }
        }
    });
    xhr.open(method, `/api/v1${url}`);
    xhr.setRequestHeader('content-type', 'application/json');
    xhr.setRequestHeader('user-token', localStorage.getItem('user-token'));
    return xhr;
}

/**
 * Gets the title from the HTML with "name" as ID, and calls the create API endpoint with it.
 */
function createItem() {
    const title = document.getElementById("name");
    const url = `/item/create/${title.value}`;
    const call = apiCall(url, "POST");
    call.send();
    document.getElementById("name").value = null;
}

/**
 * Calls the get items API.
 */
function getItems() {
    const call = apiCall("/item/get", "GET");
    call.send();
}

/**
 * Gets the title from this, and calls the edit API endpoint.
 */
function editItem() {
    const title = this.id.replaceAll("-", " ").replace("edit ", "");
    const call = apiCall("/item/edit", "PUT");
    const json = {
        title: title,
        status: "done",
    };
    call.send(JSON.stringify(json));
}

/**
 * Gets the title from this, and calls the delete API endpoint.
 */
function deleteItem() {
    const title = this.id.replaceAll("-", " ").replace("delete ", "");
    const call = apiCall("/item/delete", "POST");
    const json = {
        title,
        status: "done",
    };
    call.send(JSON.stringify(json));
}

document.getElementById("create-button").addEventListener("click", createItem);
