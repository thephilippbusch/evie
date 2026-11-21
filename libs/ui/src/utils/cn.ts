import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

/** Joins class names and lets a later Tailwind class win over a conflicting earlier one. */
export const cn = (...inputs: ClassValue[]) => twMerge(clsx(inputs));
