import "./voice-activity-visualizer.scss";
import {useEffect, useRef, useState} from "react";
import {Flex} from "antd";

export const VoiceActivityVisualizer = () => {
    const [volumes, setVolumes] = useState<number[]>([]);
    const [barCount, setBarCount] = useState<number>(0);
    const containerRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        // 监听容器宽度变化
        const resizeObserver = new ResizeObserver(() => {
            if (containerRef.current) {
                const newBarCount = Math.floor(containerRef.current.offsetWidth / 4);
                setBarCount(newBarCount);
            }
        });
        if (containerRef.current) {
            resizeObserver.observe(containerRef.current);
        }

        // 清理 observer
        return () => {
            resizeObserver.disconnect();
        };
    }, []);

    useEffect(() => {
        setVolumes(Array.from({ length: barCount }, () => 1));
        const interval = setInterval(() => {
            const newValue = Math.random() * 20 + 1;
            setVolumes((prev) => {
                return [...prev.slice(1), newValue];
            });
        }, 20);
        return () => clearInterval(interval);
    }, [barCount]);

    return (
        <Flex justify={"flex-start"} align={"center"} gap={2} className="voice-activity-visualizer" ref={containerRef}>
            {volumes.map((v, i) => (
                <div key={i} className="bar" style={{ height: `${v}px` }} />
            ))}
        </Flex>
    );
};
