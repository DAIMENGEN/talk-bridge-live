import "./voice-activity-visualizer.scss";
import {useEffect, useState} from "react";

export const VoiceActivityVisualizer = () => {
    const BAR_COUNT = 100;
    const [volumes, setVolumes] = useState<number[]>(
        Array.from({ length: BAR_COUNT }, () => 1)
    );
    useEffect(() => {
        const interval = setInterval(() => {
            const newValue = Math.random() > 0.8 ? Math.random() * 20 + 10 : Math.random() * 4 + 1;
            setVolumes((prev) => {

                return [...prev.slice(1), newValue];
            });
        }, 20);
        return () => clearInterval(interval);
    }, []);
    return (
        <div className="voice-activity-visualizer">
            {volumes.map((v, i) => (
                <div key={i} className="bar" style={{ height: `${v}px` }} />
            ))}
        </div>
    );
}