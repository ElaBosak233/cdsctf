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
        </div>
    );
}

export { Background };
