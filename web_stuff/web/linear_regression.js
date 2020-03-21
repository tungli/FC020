const js = import("./node_modules/@tungli/fc020_lr_wasm/fc020_lr_wasm.js");

var Plotly = require('plotly.js');

function plot() {

    let sigma = document.getElementById('sigma').value;
    let seed = BigInt(document.getElementById('seed').value);

    js.then(js => {
        let data = js.NoisyParabola.new(sigma, seed);
        let x = data.x();
        let y = data.y();

        var trace = {
          x: x,
          y: y,
          mode: 'markers',
          marker: {
            color: 'black'
          },
          type: 'scatter',
          name: 'Data'
        };

        console.log(data.x());
        console.log(data.y());

        var layout = {
          title: 'Parabola linear regression',
          autosize: true
        };
    
        Plotly.newPlot('gd', [trace], layout);
    });
}

function fit() {

    let sigma = document.getElementById('sigma').value;
    let seed = BigInt(document.getElementById('seed').value);
    let alpha = document.getElementById('alpha').value;

    js.then(js => {
        let data = js.NoisyParabola.new(sigma, seed);
        let x = data.x();
        let y = data.y();


        let linReg = js.LinRegParabola.new(alpha, data);
        let out = linReg.calc();


        let params = out.params();
        let sigmas = out.sigmas();
        let belt_l = out.belt_l();
        let belt_u = out.belt_u();
        let model = out.predicted();
        let sigma_l = out.sigma_l;
        let sigma_u = out.sigma_u;
        let rss = out.rss;
        let correlation = out.correlation;

        document.getElementById("stats").innerHTML = `
            Parameters: <br>
            &emsp; a = ${params[0]} +/- ${sigmas[0]}, <br>
            &emsp; b = ${params[1]} +/- ${sigmas[1]} <br>

            Correlation coefficient: ${correlation} <br>
            &sigma;(y) est.: ${sigma_l} -- ${sigma_u} <br>
            Sum of squares: ${rss} <br>`;

        var trace1 = {
          x: x,
          y: y,
          mode: 'markers',
          marker: {
            color: 'black'
          },
          type: 'scatter',
          name: 'Data'
        };

        var trace2 = {
          x: x,
          y: belt_l,
          line: {
            color: 'blue',
            dash: 'dashdot'
          },
          mode: 'lines',
          name: 'Lower'
        };

        var trace3 = {
          x: x,
          y: belt_u,
          line: {
            color: 'blue',
            dash: 'dashdot'
          },
          mode: 'lines',
          name: 'Upper'
        };

        var trace4 = {
          x: x,
          y: model,
          line: {
            color: 'red',
            dash: 'solid'
          },
          mode: 'lines',
          name: 'Model'
        };

        var layout = {
          title: 'Parabola linear regression',
          autosize: true
        };
    
        Plotly.newPlot('gd', [trace1, trace2, trace3, trace4], layout);
    });
}


function run_tests() {

    let sigma = document.getElementById('sigma').value;
    let alpha = document.getElementById('alpha').value;
    let seed = BigInt(document.getElementById('seed').value);

    let n_tests = 1000;
    js.then(js => {
        let tests = js.TestIntervals.new(alpha,  sigma, n_tests, seed);
        let out = tests.test();

        let a = Math.round(10000*out.param_a)/100;
        let b = Math.round(10000*out.param_b)/100;
        let sigma_y = Math.round(10000*out.sigma_y)/100;
        let belt = Math.round(10000*out.belt)/100;

        document.getElementById("tests_results").innerHTML = `
            Number of tests is set to ${n_tests}.<br><br>
            Parameter 'a' guessed correctly ${a}% times.<br>
            Parameter 'b' guessed correctly ${b}% times.<br>
            Whole interval guessed correctly ${belt}% times.<br>
            &sigma;(y) guessed correctly ${sigma_y}% times.<br>`
    });
}


document.getElementById("plot").onclick = plot;
document.getElementById("fit").onclick = fit;
document.getElementById("tests").onclick = run_tests;

plot();
