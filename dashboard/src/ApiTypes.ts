import { z } from "zod";

export const apiUrl = "http://localhost:3000/api/v1";

export const ApiFunction = z
  .object({
    function: z.string().min(1),
    signature: z.string().min(1),
  })
  .required();

export const ApiModule = z.object({
  id: z.number(),
  module_hash: z.string(),
  functions: z.array(ApiFunction),
});

export const FunctionResult = z.object({
  return_value: z.array(z.number()),
});

export const ApiResponse = z.object({
  modules: z.array(ApiModule),
});

export const LoginReponse = z.object({
  jwt: z.string().min(1),
});

export type ApiFunctionType = z.infer<typeof ApiFunction>;
export type ApiModuleType = z.infer<typeof ApiModule>;
export type ApiResponseType = z.infer<typeof ApiResponse>;
export type LoginReponseType = z.infer<typeof LoginReponse>;
export type FunctionResultType = z.infer<typeof FunctionResult>;
