import * as d3 from "d3";
import { useEffect, useRef } from "react";
import { GraphInterpolation } from "./GraphInterpolation";
import { GraphDatum } from "./GraphDatum";


interface NumberGraphProps<T> {
  datum: GraphDatum<T, number>
  width?: number,
  height?: number,
  margin?: { top: number, bottom: number, left: number, right: number },
  interpolation?: GraphInterpolation,
  unit?: string,
  timeDomainMs?: number
  refreshRate? : number
  bounds? : {max : number, min : number},
  timeShiftMs? : number
}


function NumberGraph<T>({
  datum,
  width = 400,
  height = 200,
  margin = { top: 0, bottom: 0, left: 0, right: 0 },
  interpolation = GraphInterpolation.Step, 
  unit,
  timeDomainMs = 5000,
  refreshRate = 500,
  bounds,
  timeShiftMs = 0,
}: NumberGraphProps<T>) {
  margin.left += 40;
  margin.bottom += 20;

  const svgRef = useRef<SVGSVGElement>(null);

  useEffect(() => {

    const innerWidth = width - margin.left - margin.right;
    const innerHeight = height - margin.top - margin.bottom;

    const xScale = d3.scaleLinear().range([0, innerWidth]);
    const yScale = d3.scaleLinear().range([innerHeight, 0]);
    const line = d3.line();
    // Select interpolation mode!
    switch (interpolation) {
      case GraphInterpolation.BasisSpline:
        line.curve(d3.curveBasis);
        break;
      case GraphInterpolation.BezierCurve:
        line.curve(d3.curveBumpX);
        break;
      case GraphInterpolation.CardinalSpline:
        line.curve(d3.curveCardinal);
        break;
      case GraphInterpolation.CatmullRomSpline:
        line.curve(d3.curveCatmullRom);
        break;
      case GraphInterpolation.Linear:
        line.curve(d3.curveLinear);
        break;
      case GraphInterpolation.MonotoneSpline:
        line.curve(d3.curveMonotoneX);
        break;
      case GraphInterpolation.NaturalSpline:
        line.curve(d3.curveNatural);
        break;
      case GraphInterpolation.Step:
        line.curve(d3.curveStepAfter);
        break;
    }
    // access the properties!
    line.x((d: any) => xScale(datum.xValue(d)));
    line.y((d: any) => yScale(datum.yValue(d)));


    const timeAxis = d3.axisBottom(xScale)
      .tickFormat(x => `${x as number / 1000.0}s`)
      .ticks(5);

    const yAxis = d3.axisLeft(yScale).tickFormat(x => `${x}${unit ?? ""}`);

    const yAxisGrid = d3.axisLeft(yScale)
      .tickSize(-innerWidth)
      .tickFormat("" as any)
      .ticks(5);


    const svg = d3.select(svgRef.current)
      .attr("width", width)
      .attr("height", height)
      .append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`)

    svg.append("defs")
      .append("clipPath")
      .attr("id", "clip")
      .append("rect")
      .attr("width", innerWidth)
      .attr("height", innerHeight);

    const graph = svg.append("g");
    graph.attr("clip-path", "url(#clip)");

    const a = svg.append("g")
      .attr("transform", `translate(0 ${innerHeight})`)
      .call(timeAxis);

    const b = svg.append("g")
      .attr("transform", "translate(0 0)")
      .call(yAxis);

    const c = graph.append("g")
      .style("color", "#afcdfa")
      .style("stroke-width", 0.5)
      .call(yAxisGrid);

    const value = svg.append("text")
      .attr("x", innerWidth - 20)
      .attr("y", 25)
      .attr("text-anchor", "end");



    const group = graph.append("g");

    group.append("path")
      .datum(datum.values)
      .attr("d", line(datum.values as any))
      .attr("class", "line")
      .style("stroke", "#ff1e00")
      .style("stroke-width", 1.5)
      .style("fill", "none")

    let running = true;

    // UPDATE METHODS

    // initalize timestamp
    let timestamp = datum.xValue(datum.values[datum.values.length - 1]);

    let maxY = bounds?.max ?? Math.max(...datum.values.map(datum.yValue));
    let minY = bounds?.min ?? Math.min(...datum.values.map(datum.yValue));

    function updateXScale() {
      xScale.domain([timestamp - timeDomainMs, timestamp - timeShiftMs])
    }

    function updateYScale() {
      yScale.domain([minY, maxY * 1.25]);
    }


    function updateSvgLineGraph() {
      group.attr("transform", "");
      group.select(".line")
        .datum(datum.values as any)
        .attr("d", line);
    }

    function updateTextValue() {
      value.text(`${Math.round(
        datum.yValue(datum.values[datum.values.length - 1]) * 100.0)
        / 100.0}${unit ?? ""}`);
    }



    function updateSvg() {
      timestamp += refreshRate;
      updateXScale();
      updateYScale();
      updateSvgLineGraph();
      updateTextValue();
      a.transition()
        .duration(refreshRate)
        .ease(d3.easeLinear)
        .call(timeAxis);
      b.call(yAxis);
      c.call(yAxisGrid);

      if (running) {
        //restart animation
        d3.active(group.node())
          ?.transition()
          .on("start", updateSvg)
          .attr("transform", `translate(${xScale(timestamp - timeDomainMs - refreshRate)},0)`);
      }
    }
    
    group.transition()
      .on("start", updateSvg)
      .ease(d3.easeLinear)
      .duration(refreshRate);

    

    return () => {
      // Cleanup!
      running = false;
      svg.remove();
    }
  }, [width, height]);

  return <svg ref={svgRef}></svg>;
}

export default NumberGraph;
