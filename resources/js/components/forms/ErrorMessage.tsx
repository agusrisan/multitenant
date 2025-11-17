interface ErrorMessageProps {
  errors?: string[]
}

export function ErrorMessage({ errors }: ErrorMessageProps) {
  if (!errors || errors.length === 0) return null

  return (
    <div className="text-sm text-red-500 space-y-1">
      {errors.map((error, index) => (
        <p key={index}>{error}</p>
      ))}
    </div>
  )
}
