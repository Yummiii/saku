using System.Text.RegularExpressions;
using AutoMapper;
using Lina.DynamicMapperConfiguration.Abstracts;
using Saku.Models;
using Saku.ViewModels;

namespace Saku.Mappers;

public class UserRegisterViewModelToUserModelMapper : DynamicMapperProfile<UserRegisterViewModel, UserModel>
{
    protected override void Map(IMappingExpression<UserRegisterViewModel, UserModel> mappingExpression)
    {
        mappingExpression.ForCtorParam(nameof(UserModel.State), x => x.MapFrom(_ => UserState.Normal));
        mappingExpression.ForCtorParam(nameof(UserModel.UserName), x => x.MapFrom(y => CleanUsername(y.UserName)));
    }
    
    private static string CleanUsername(string username)
    {
        var cleanUsername = Regex.Replace(username, @"[^a-zA-Z0-9_-]", string.Empty);
        if (cleanUsername.Length > 64)
        {
            cleanUsername = cleanUsername[..64];
        }

        return cleanUsername;
    }
}