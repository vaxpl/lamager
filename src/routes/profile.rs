use crate::ldap::LdapAccessor;
use crate::models::{NewPassword, Person, SessionRef};
use image::imageops::FilterType as ImageFilterType;
use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use rocket::http::ContentType;
use rocket::request::Form;
use rocket::response::Redirect;
use rocket::{Data, Route};
use rocket_contrib::templates::Template;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
use std::collections::HashMap;
// use rocket::response::Responder;
use rocket::response::status::BadRequest;

#[get("/profile")]
pub(crate) fn profile(session: SessionRef, mut ldap: LdapAccessor) -> Template {
    let mut context: HashMap<String, String> = HashMap::new();
    // Fetch user informations
    if let Ok(entry) = ldap.entry_of_username(&session.uid) {
        // let dn = &entry.dn;
        let uid = &entry.attrs["uid"][0];
        let cn = &entry.attrs["cn"][0];
        let mail = &entry.attrs["mail"][0];
        let photo = entry
            .attrs
            .get("photo")
            .map(|x| base64::encode(x[0].as_bytes()))
            .or_else(|| entry.bin_attrs.get("photo").map(|x| base64::encode(&x[0])));
        let create_timestamp = &entry.attrs["createTimestamp"][0];
        let create_date =
            chrono::NaiveDate::parse_from_str(create_timestamp, "%Y%m%d%H%M%S%.3fZ").unwrap();
        context.insert("uid".to_string(), uid.to_string());
        context.insert("cn".to_string(), cn.to_string());
        context.insert("mail".to_string(), mail.to_string());
        if let Some(photo) = photo {
            context.insert("photo".to_string(), photo);
        }
        context.insert("createTimestamp".to_string(), format!("{}", create_date));
    }
    // Render the page
    Template::render("profile", &context)
}

#[get("/profile", rank = 2)]
pub(crate) fn profile_without_session() -> Redirect {
    Redirect::to(uri!(crate::routes::login::login_page))
}

#[post("/profile/avatar", data = "<data>")]
pub(crate) fn profile_avatar(
    content_type: &ContentType,
    data: Data,
    session: SessionRef,
    mut ldap: LdapAccessor,
) -> Result<(), BadRequest<String>> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("avatar_file")
            .content_type_by_string(Some(mime::IMAGE_JPEG))
            .map_err(|err| BadRequest(Some(err.to_string())))?,
        MultipartFormDataField::text("avatar_file_name"),
    ]);

    let multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .map_err(|err| BadRequest(Some(err.to_string())))?;

    if let Some(files) = multipart_form_data.files.get("avatar_file") {
        let file = &files[0];
        // let _content_type = &file.content_type;
        // let _file_name = &file.file_name;
        let path = &file.path;

        let image = ImageReader::open(path)
            .map_err(|err| BadRequest(Some(err.to_string())))?
            .with_guessed_format()
            .map_err(|err| BadRequest(Some(err.to_string())))?
            .decode()
            .map_err(|err| BadRequest(Some(err.to_string())))?;

        let scaled = image.resize_to_fill(512, 512, ImageFilterType::Triangle);
        let mut buffer = Vec::new();
        scaled
            .write_to(&mut buffer, ImageOutputFormat::Jpeg(80))
            .map_err(|err| BadRequest(Some(err.to_string())))?;

        ldap.update_photo(&session.dn, &buffer)
            .map_err(|err| BadRequest(Some(err.to_string())))?;
    }

    Ok(())
}

#[post("/profile/password", data = "<new_password>")]
pub(crate) fn profile_password(
    new_password: Form<NewPassword>,
    session: SessionRef,
    mut ldap: LdapAccessor,
) -> Result<(), BadRequest<String>> {
    ldap.update_password(&session.dn, &new_password.into_inner())
        .map_err(|err| BadRequest(Some(err.to_string())))?;
    Ok(())
}

#[post("/profile/person", data = "<person>")]
pub(crate) fn profile_person(
    person: Form<Person>,
    session: SessionRef,
    mut ldap: LdapAccessor,
) -> Result<(), BadRequest<String>> {
    ldap.update_person(&session.dn, &person.into_inner())
        .map_err(|err| BadRequest(Some(err.to_string())))?;
    Ok(())
}

pub fn routes() -> Vec<Route> {
    routes![
        profile,
        profile_without_session,
        profile_avatar,
        profile_password,
        profile_person,
    ]
}
