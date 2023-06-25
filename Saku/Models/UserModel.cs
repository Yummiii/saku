namespace Saku.Models;

public class UserModel
{
    public UserModel(ulong discordId, string userName, UserState state)
    {
        DiscordId = discordId;
        UserName = userName;
        State = state;
    }

    public int Id { get; set; }
    public ulong DiscordId { get; set; }
    public string UserName { get; set; }
    public UserState State { get; set; }

    public IEnumerable<ChatContextModel>? ChatContexts { get; set; }
}