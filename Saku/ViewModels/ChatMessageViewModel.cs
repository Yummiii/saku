using Saku.Models;

namespace Saku.ViewModels;

public record ChatMessageViewModel(
    ChatType ChatType,
    string Message,
    DateTime CreatedAt,
    string? UserName = null
);