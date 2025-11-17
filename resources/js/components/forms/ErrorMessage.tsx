interface ErrorMessageProps {
  id?: string
  message?: string
  errors?: string[]
}

export default function ErrorMessage({ id, message, errors }: ErrorMessageProps) {
  // Single message mode
  if (message) {
    return (
      <p id={id} className="text-sm text-red-500 mt-1">
        {message}
      </p>
    )
  }

  // Multiple errors mode
  if (!errors || errors.length === 0) return null

  return (
    <div className="text-sm text-red-500 space-y-1">
      {errors.map((error, index) => (
        <p key={index}>{error}</p>
      ))}
    </div>
  )
}

// Named export for backward compatibility
export { ErrorMessage }
