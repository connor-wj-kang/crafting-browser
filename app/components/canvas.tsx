"use client";
import HtmlParser from "@/lib/html-parser";
import { useEffect } from "react";

const Canvas = () => {
  useEffect(() => {
    const result = new HtmlParser(
      "<script>as</script><p>4-4<big>Quoted attributes</big>. Quoted attributes can contain spaces and right angle brackets. Fix the lexer so that this is supported properly. Hint: the current lexer is a finite state machine, with two states (determined by <code>in_tag</code>). You’ll need more states.</p>",
    ).parse();
    console.log(result);
  }, []);

  return (
    <canvas className="bg-white" id="canvas" width={800} height={600}></canvas>
  );
};

export default Canvas;
