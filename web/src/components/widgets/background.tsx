import { useEffect, useRef } from "react";

function Background() {
    const svgPath1 = useRef<SVGPathElement>(null);
    const svgPath2 = useRef<SVGPathElement>(null);

    useEffect(() => {
        const timer = setTimeout(() => {
            svgPath1.current?.classList.remove("draw-line");
            svgPath2.current?.classList.remove("draw-line");
        }, 3000);

        return () => clearTimeout(timer);
    }, []);

    return (
        <div className="fixed -left-1 -right-1 -top-1 -bottom-1 -z-10 print:hidden">
            <div className="fixed left-0 right-0 top-0 bottom-0 bg-layer/90 transition-colors duration-700" />
            {/* Svg here */}
            <svg
                viewBox="0 0 960 1080"
                xmlns="http://www.w3.org/2000/svg"
                className="fixed left-0 bottom-0 h-screen opacity-15 print:hidden"
            >

            <g className="opacity-30">
            {Array.from({ length: 10 }).map((_, i) => (
                <line
                key={`grid-h-${i}`}
                x1="0"
                y1={i * 108}
                x2="960"
                y2={i * 108}
                stroke="currentColor"
                strokeWidth="1"
                />
            ))}
            {Array.from({ length: 10 }).map((_, i) => (
                <line
                key={`grid-v-${i}`}
                x1={i * 96}
                y1="0"
                x2={i * 96}
                y2="1080"
                stroke="currentColor"
                strokeWidth="1"
                />
            ))}
            </g>

            <circle cx="150" cy="150" r="8" fill="gray" className="opacity-70" />
            <circle cx="350" cy="150" r="8" fill="gray" className="opacity-70" />
            <circle cx="550" cy="150" r="8" fill="gray" className="opacity-70" />
            <circle cx="150" cy="930" r="8" fill="gray" className="opacity-70" />
            <circle cx="350" cy="930" r="8" fill="gray" className="opacity-70" />
            <circle cx="550" cy="930" r="8" fill="gray" className="opacity-70" />

            {Array.from({ length: 15 }).map((_, i) => (
            <circle
                key={`dot-l-${i}`}
                cx={100 + Math.floor(Math.random() * 500)}
                cy={100 + Math.floor(Math.random() * 880)}
                r="3"
                fill="gray"
                className="opacity-50"
            />
            ))}

                <circle
                cx="250"
                cy="400"
                r="20"
                fill="none"
                stroke="gray"
                strokeWidth="1"
                className="animate-ping opacity-30"
                />
                <circle
                cx="450"
                cy="680"
                r="20"
                fill="none"
                stroke="gray"
                strokeWidth="1"
                className="animate-ping opacity-30"
                />
            </svg>

            <svg
                viewBox="0 0 960 1080"
                xmlns="http://www.w3.org/2000/svg"
                className="fixed right-0 top-0 h-screen opacity-15 print:hidden"
            >
                <g className="opacity-30">
                {Array.from({ length: 10 }).map((_, i) => (
                    <line
                    key={`grid-h-${i}`}
                    x1="0"
                    y1={i * 108}
                    x2="960"
                    y2={i * 108}
                    stroke="currentColor"
                    strokeWidth="1"
                    />
                ))}
                {Array.from({ length: 10 }).map((_, i) => (
                    <line
                    key={`grid-v-${i}`}
                    x1={i * 96}
                    y1="0"
                    x2={i * 96}
                    y2="1080"
                    stroke="currentColor"
                    strokeWidth="1"
                    />
                ))}
                </g>

                <path
                d="M 200,200 L 230,250 L 170,250 Z"
                fill="gray"
                className="opacity-50"
                />
                <path
                d="M 760,700 L 730,650 L 790,650 Z"
                fill="gray"
                className="opacity-50"
                />

                {Array.from({ length: 12 }).map((_, i) => (
                <rect
                    key={`rect-r-${i}`}
                    x={310 + Math.floor(Math.random() * 340)}
                    y={150 + Math.floor(Math.random() * 780)}
                    width="6"
                    height="6"
                    fill="gray"
                    className="opacity-50"
                />
                ))}

                <rect
                x="480"
                y="325"
                width="30"
                height="30"
                fill="none"
                stroke="gray"
                strokeWidth="1"
                className="animate-ping opacity-30"
                />
                <rect
                x="480"
                y="775"
                width="30"
                height="30"
                fill="none"
                stroke="gray"
                strokeWidth="1"
                className="animate-ping opacity-30"
                />
            </svg>

            <svg
                viewBox="0 0 1920 1080"
                xmlns="http://www.w3.org/2000/svg"
                className="fixed left-0 top-0 h-screen w-screen opacity-10 print:hidden"
            >
                <path
                d="M 650,540 L 1270,540 M 650,340 L 1270,340 M 650,740 L 1270,740"
                stroke="currentColor"
                strokeWidth="1"
                strokeDasharray="10,10"
                className="animate-pulse"
                />
            </svg>
        </div>
    );
}

export { Background };
