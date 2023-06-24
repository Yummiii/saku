using Discord.Interactions;
using Saku.Adapters.Interfaces;
using Saku.ViewModels;

namespace Saku.Modules;

public class TestModule : InteractionModuleBase
{
    private readonly IOpenIaAdapter _openIaAdapter;

    public TestModule(IOpenIaAdapter openIaAdapter)
    {
        _openIaAdapter = openIaAdapter;
    }

    [SlashCommand("pitas", "manda mensagem fofa")]
    public async Task SendPitasGay(string message)
    {
        var messages = new[] { new ChatMessageViewModel(ChatType.User, message) };
        var result = await _openIaAdapter.SendChat(messages);

        await RespondAsync(result.Last().Message);
    }
}