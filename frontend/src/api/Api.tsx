const API_URL = "http://localhost:8000/api"

type GroupName = {
    key: string
    display_name: string
}

export async function getGroupNames(): Promise<GroupName[]> {
    return await fetch(`${API_URL}/groups`)
        .then(res => res.json())
        .catch(console.log);
}

type ChallengeName = {
    key: string
    display_name: string
}

type GroupInfo = {
    name: string
    url: string
    challenges: ChallengeName[]
}

export async function getGroupInfo(key: string): Promise<GroupInfo> {
    return await fetch(`${API_URL}/groups/${key}`)
        .then(res => res.json())
        .catch(console.log);
}