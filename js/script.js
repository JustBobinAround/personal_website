function check_and_clear() {
    const is_mobile = /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
    const is_small_screen = window.innerWidth < 760;

    if (is_mobile) {
        document.body.innerHTML = '';

        const message = document.createElement('div');
        message.style.fontSize = '24px';
        message.style.textAlign = 'center';
        message.style.padding = '50px';
        message.innerText = 'This site is for software engineers, please use a PC like a normal SWE would.';
        document.body.appendChild(message);
        return false;
    }
    return true;
}
function screen_size_changed() {
    if(check_and_clear()) {
        let element = document.getElementById('grid-container');

        if(!(element==null)){
            element.remove();
        }

        const { char_width, char_height } = get_char_dim();

        const grid_width = window.innerWidth-1;
        const grid_height = window.innerHeight-1;

        const div_count_x = Math.floor(grid_width / char_width)-8; 
        const div_count_y = Math.floor(grid_height / char_height)-1; 

        console.log(`Window size changed to ${grid_width}x${grid_height}`);

        create_grid(div_count_x, div_count_y, grid_width, grid_height);
    } else {
        let element = document.getElementById('grid-container');

        if(!(element==null)){
            element.remove();
        }
    }
}
function create_grid(count_x, count_y, totalWidth, totalHeight) {
    let test = {"lines":[{"chars":[{"is_bold":true,"is_ital":false,"is_cursor":false,"is_highlight":false,"is_link":true,"char":"l","color":"Red","link":"./files/hash1"},{"is_bold":true,"is_ital":false,"is_cursor":false,"is_highlight":false,"is_link":true,"char":"m","color":"Red","link":"./files/hash1"},{"is_bold":true,"is_ital":false,"is_cursor":false,"is_highlight":false,"is_link":true,"char":"a","color":"Red","link":"./files/hash1"},{"is_bold":true,"is_ital":false,"is_cursor":false,"is_highlight":false,"is_link":true,"char":"o","color":"Red","link":"./files/hash1"}]}]};
    const grid_countainer = document.createElement('div');

    grid_countainer.setAttribute('id', 'grid-container');
    grid_countainer.style.padding = 0;
    grid_countainer.style.margin = 0;
    grid_countainer.style.width = `${totalWidth}px`;
    grid_countainer.style.height = `${totalHeight}px`;
    grid_countainer.style.display = 'grid';
    grid_countainer.style.gridTemplateColumns = `repeat(${count_x}, 1fr)`;
    grid_countainer.style.gridTemplateRows = `repeat(${count_y}, 1fr)`;

    var buffer_count = 0;

    var test_data = test.lines;

    for(let y = 0; y < count_y; y++) {
        var line = test_data[(y+1-count_y)+test_data.length];
        for (let x = 0; x < count_x; x++) {
            const div = document.createElement('div');
            div.classList.add('char-box');
            if(x<5&&y<count_y-1) {
                var y_str = y.toString();
                if(x>=5-y_str.length-1) {
                    div.innerText = y_str.charAt(x-(5-y_str.length-1));
                } else {
                    div.innerText = ' ';
                }
            } else if(line==null) {
                if(y==count_y-1) {
                    div.innerText = '=';
                } else {
                    div.innerText = ' ';
                }
            } else {
                if(x-5<line.chars.length) {
                    div.innerText = line.chars[x-5].char;
                } else {
                    div.innerText = ' ';
                }
            }
 
            grid_countainer.appendChild(div);

        }
    }

    document.body.appendChild(grid_countainer);
}

function get_char_dim() {
    var bodyElement = document.body;
    var computed_style = window.getComputedStyle(bodyElement);
    var font_size = computed_style.getPropertyValue('font-size');

    const test_div = document.createElement('div');
    test_div.style.fontFamily = 'monospace';
    test_div.style.fontSize = font_size;
    test_div.style.position = 'absolute';
    test_div.style.whiteSpace = 'pre';
    test_div.style.visibility = 'hidden';
    test_div.textContent = 'M';
    document.body.appendChild(test_div);

    const char_width = test_div.clientWidth-1;
    const char_height = test_div.clientHeight;
    document.body.removeChild(test_div);

    return { char_width, char_height };
}
document.addEventListener('keydown', function(event) {
    if (event.key.startsWith('F') && !isNaN(event.key.substring(1))) {
        console.log('Function key pressed:', event.key);
        
        event.preventDefault();
    }
});

window.onload = function() {
    screen_size_changed();
    window.addEventListener('resize', screen_size_changed);
};
