import { useLibreInfo } from "../hooks/librespot"

export const LibreStatus = () => {
    const { data, error, status } = useLibreInfo();
    const color = !error && status === 'success' && data?.status == "Running" ? 'green' : 'red';
    let statusCircle = <circle r="5" cx="50%" cy="50%" fill={color}></circle>
    return (
        <div className="d-flex justify-content-center align-items-center">
            { error || status !== "success" ? <div>Error Fetching Status</div>
                : <div>{ data?.status === "Running" ? 'Service is Running!' : 'Stopped' }</div>
            }
            <svg  viewBox="0 0 15 15 " width="25px" height="25px" className="d-flex justify-content-center align-items-center">
                { statusCircle }
            </svg>
        </div>
    )
}