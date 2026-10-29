import type { UserNewsletterRegisterRequest } from '@user/user_types';
import { apiFetch } from '@utils/api_client';

export const postUserRegisterNewsletter = (
  data: UserNewsletterRegisterRequest,
) => {
  return apiFetch<void>('/subscriptions', {
    method: 'POST',
    body: JSON.stringify(data),
  });
};
