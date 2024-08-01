import { User } from "@/types/user";

export interface Team {
    id?: number;
    name?: string;
    description?: string;
    email?: string;
    captain_id?: number;
    captain?: User;
    is_locked?: boolean;
    created_at?: string;
    updated_at?: string;
    users?: Array<User>;
}

export interface TeamFindRequest {
    id?: number;
    name?: string;
    captain_id?: number;
    page?: number;
    size?: number;
    sort_key?: string;
    sort_order?: string;
}

export interface TeamUpdateRequest {
    id?: number;
    name?: string;
    description?: string;
    email?: string;
    captain_id?: number;
    is_locked?: boolean;
}

export interface TeamCreateRequest {
    name: string;
    description: string;
    email?: string;
    captain_id: number;
}

export interface TeamDeleteRequest {
    id: number;
}

export interface TeamJoinRequest {
    id: number;
    token: string;
}

export interface TeamLeaveRequest {
    id: number;
}

export interface TeamGetInviteTokenRequest {
    id: number;
}

export interface TeamUpdateInviteTokenRequest {
    id: number;
}

export interface TeamDeleteUserRequest {
    id: number;
    user_id: number;
}
