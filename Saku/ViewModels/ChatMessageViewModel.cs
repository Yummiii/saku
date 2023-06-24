namespace Saku.ViewModels;

public record ChatMessageViewModel(
    ChatType ChatType,
    string Message,
    string? UserName = null
);