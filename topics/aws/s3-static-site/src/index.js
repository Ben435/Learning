function component() {
    const el = document.createElement('div');

    const button = document.createElement('button');
    button.onclick = ev => {
        fetch('https://api.bens-stuff.net/exp').then(console.log);
    };
    button.innerText = "Go!";

    el.appendChild(button);

    return el;
}

document.getElementById('app').appendChild(component());
