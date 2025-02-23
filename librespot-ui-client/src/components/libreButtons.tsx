import { useQueryClient } from "@tanstack/react-query"
import { useCallback } from "react";

export const LibreButtons = () => {
    const client = useQueryClient();

    const startService = useCallback(async () => {
        await fetch('/api/librespot/start');
        client.invalidateQueries({ queryKey: ['info'] });
        client.refetchQueries({ queryKey: ['info'] });
    }, []);

    const stopService = useCallback(async () => {
        await fetch('/api/librespot/stop');
        client.invalidateQueries({ queryKey: ['info'] });
        client.refetchQueries({ queryKey: ['info'] });
    }, []);

    return (
        <div className="d-flex">
            <div className="px-3"><button type="button" className="btn btn-primary" onClick={startService}>Start Service</button></div>
            <div><button type="button" className="btn btn-danger" onClick={stopService}>Stop Service</button></div>
        </div>
    )
}