import { useLibreInfo } from "../hooks/librespot"

export const LibreStatus = () => {
    const { data } = useLibreInfo();
    return (
        <div>
            { JSON.stringify(data) }
        </div>
    )
}