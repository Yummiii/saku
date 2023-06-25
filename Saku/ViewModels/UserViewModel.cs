using Saku.Models;

namespace Saku.ViewModels;

public record UserViewModel(
    int Id, 
    ulong DiscordId,
    string UserName,
    UserState State);