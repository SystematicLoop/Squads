function Listbox(hostElement) {
    // Properties.
    this.hostElement = hostElement;
    this.items = [];
    this.selection = null;

    // Methods.
    this.add = add;
    this.remove = remove;
    this.clear = clear;
    this.indexOf = indexOf;
    this.setSelection = setSelection;

    return this;
}

function add(text, data) {
    var index = this.items.length;
    var child = document.createElement('div');
    child.classList.add('definition-item');
    child.textContent = text;
    
    {
        let listbox = this;
        child.onclick = function() {
            let index = listbox.indexOf(child);
            listbox.setSelection(index);
        }
    }

    this.hostElement.appendChild(child);

    let item = {
        element: child,
        data: data,
    };

    this.items.push(item);

    return index;
}

function remove(index) {
    if (index != null) {
        var item = this.items[index];
    
        this.hostElement.removeChild(item.element);
        this.items.splice(index, 1);

        if (index == this.selection) {
            this.selection = null;
        }
    }
}

function clear() {
    while (this.hostElement.firstChild) {
        this.hostElement.removeChild(this.hostElement.firstChild);
    }

    this.items.splice(0, this.items.length);
    this.selection = null;
}

function indexOf(element) {
    for (let i = 0; i < this.items.length; i++) {
        let item = this.items[i];
        if (item.element == element) {
            return i;
        }
    }

    return null;
}

function onItemClick(event) {
    setSelection(this);
}

function setSelection(index) {
    if (this.selection != null) {
        var item = this.items[this.selection];

        item.element.style.background = null;
        item.element.style.color = null;
    }
    
    this.selection = index;

    if (this.selection != null) {
        let background = getComputedStyle(document.documentElement)
            .getPropertyValue('--list-item-selected-background-color');
       
        if (background.length == 0) {
            background = '#3390ff';
        }
    
        let foreground = getComputedStyle(document.documentElement)
            .getPropertyValue('--list-item-selected-foreground-color');

        if (foreground.length == 0) {
            foreground = '#ffffff';
        }

        var item = this.items[this.selection];
        item.element.style.backgroundColor = background;
        item.element.style.color = foreground;
    }
}