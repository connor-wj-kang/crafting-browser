"use client";
import init, { start } from "@/pkg/browser";
import { useEffect } from "react";

const Canvas = () => {
  useEffect(() => {
    init().then(() => start());
  }, []);

  return (
    <canvas className="bg-white" id="canvas" width={800} height={600}></canvas>
  );
};

export default Canvas;
