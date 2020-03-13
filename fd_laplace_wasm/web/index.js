const js = import("./node_modules/@tungli/fd_laplace_wasm/fd_laplace_wasm.js");

var Plotly = require('plotly.js');

function plot() {

    let size = document.getElementById('domain_size').value

    js.then(js => {
        let surf = js.FdLaplace.new(size);
        var data = []
        for (i=0;i<surf.size;i++) {
            data.push(surf.row(i));
        }
        console.log(data);

        let x = surf.x();
        
        var data = [{
                   x: x,
                   y: x,
                   z: data,
                   type: 'surface'
                }];
          
        var layout = {
          title: 'Solution',
          autosize: true
        };
    
        Plotly.newPlot('gd', data, layout);
    });
}

function plot_error() {

    let size = document.getElementById('domain_size').value

    js.then(js => {
        let surf = js.FdLaplace.new(size);
        var data = []
        for (i=0;i<surf.size;i++) {
            data.push(surf.error_row(i));
        }
        console.log(data);

        let x = surf.x();
        
        var data = [{
                   x: x,
                   y: x,
                   z: data,
                   type: 'surface'
                }];
          
        var layout = {
          title: '|calc - analytic|',
          autosize: true
        };
    
        Plotly.newPlot('gd', data, layout);
    });
}

document.getElementById("recalc").onclick = plot;
document.getElementById("error_calc").onclick = plot_error;

plot();
