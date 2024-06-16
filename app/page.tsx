"use client";
import { useState, useEffect, useRef } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import localFont from "next/font/local";

const myFont = localFont({ src: "./JetBrainsMono-Medium.woff2" });

export default function Home() {
  const inputRef = useRef<HTMLInputElement>(null);

  const [cmd, setCmd] = useState("");
  const [path, setPath] = useState("");
  const [history, setHistory] = useState<
    { CurrentPath: string; command: string; output: string }[]
  >([]);

  useEffect(() => {
    invoke("return_path").then((res) => {
      if (!res) return;
      setPath(res.toString());
    });
  }, []);

  useEffect(() => {
    const handleWindowClick = () => {
      if (!inputRef.current) return;
      inputRef.current.focus();
    };

    const handleWindowEnter = (e: KeyboardEvent) => {
      if (e.key === "Enter") {
        if (!inputRef.current) return;
        const value = inputRef.current.value;
        invoke("execute_command", { command: value }).then((res) => {
          scrollTo(0,document.getElementById("active-input")?.offsetTop as number   );
          console.log(document.getElementById("active-input")?.offsetTop);

          if (res?.toString() === "clean") {
            console.log("clearing history");
            setHistory([]);
            return;
          }

          setHistory((prevHistory) => [
            ...prevHistory,
            {
              CurrentPath: path,
              command: value,
              output: res ? res.toString() : "",
            },
          ]);
          invoke("return_path").then((res) => {
            console.log(res);
            if (!res) return;
            setPath(res.toString());
          });
          setCmd("");
        });
        
      }
    };

    window.addEventListener("click", handleWindowClick);
    window.addEventListener("keydown", handleWindowEnter);

    return () => {
      window.removeEventListener("click", handleWindowClick);
      window.removeEventListener("keydown", handleWindowEnter);
    };
  }, [path]);

  return (
    <div className="mt-2 mx-2" onClick={() => inputRef.current?.focus()}>
      {history.map((entry, index) => (
        <div key={index} className="my-4">
          <span className="w-fit inline">
            <span className={myFont.className + " text-green-400 font-bold "}> {entry.CurrentPath}\</span>{" "}
            {entry.command}
          </span>
          <pre style={{ whiteSpace: "pre-wrap" }} className="text-gray-500 ">{entry.output}</pre>
        </div>
      ))}
      <span id="active-input" className={myFont.className + " text-green-400 font-bold "}> {path}\</span> <span className="text-yellow-400 ">{cmd}</span>
      <input
        type="text"
        ref={inputRef}
        value={cmd}
        className="bg-black border-none w-fit bg-none"
        style={{ position: "absolute", left: "-9999px" }} // Move the input off-screen
        onChange={(e) => setCmd(e.target.value)}

      />
    </div>
  );
}
