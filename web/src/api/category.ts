import {
    Category,
    CategoryCreateRequest,
    CategoryDeleteRequest,
    CategoryUpdateRequest,
} from "@/types/category";
import { api } from "@/utils/axios";

export async function getCategories() {
    return await api().get<{
        code: number;
        data: Array<Category>;
    }>("/categories");
}

export async function createCategory(request: CategoryCreateRequest) {
    return await api().post<{
        code: number;
        data: Category;
    }>("/categories", request);
}

export async function updateCategory(request: CategoryUpdateRequest) {
    return await api().put<{
        code: number;
        data: Category;
    }>(`/categories/${request.id}`, request);
}

export async function deleteCategory(request: CategoryDeleteRequest) {
    return await api().delete<{
        code: number;
    }>(`/categories/${request.id}`);
}
