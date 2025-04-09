import axios from "axios";

const factory = () => {
    const client = axios.create({
        baseURL: '',
        headers: { 'Content-Length': 'application/json' }
    })

    return client
}

export const httpClient = factory()
