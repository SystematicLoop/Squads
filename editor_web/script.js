var listbox = null;

function importFileDialog() {
    var input = document.createElement('input');
    input.accept = '.json';
    input.type = 'file';
    input.multiple = false;

    input.onchange = _ => {
        let file = input.files[0];
        importFile(file);
    }

    input.click();
}

function exportFileDialog() {
    
}

function importFile(file) {
    var reader = new FileReader();
    reader.readAsDataURL(file);
    reader.onload = e => {
        var url = e.target.result;

        $.getJSON(url, null, function(result) {
            for (weapon of result) {
                listbox.add(weapon.name, 0);
            }
        });

        // Do something...
    }
}

function onLoad() {
    var listbox_host_element = document.querySelector('.definitions');
    listbox = new Listbox(listbox_host_element);
}