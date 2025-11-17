import { useForm as useInertiaForm } from '@inertiajs/react'

export function useForm<T extends Record<string, any>>(initialValues: T) {
  const form = useInertiaForm(initialValues)

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const key = e.target.name
    const value = e.target.value
    form.setData(key as any, value as any)
  }

  return {
    ...form,
    handleChange,
  }
}
